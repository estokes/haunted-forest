use burn::nn::{
    Embedding, EmbeddingConfig, Linear, LinearConfig,
    RmsNorm, RmsNormConfig, RotaryEncoding, RotaryEncodingConfig,
};
use burn::prelude::*;
use burn::tensor::activation::{silu, softmax};

use crate::qwen3::ModelConfig;
use super::ste::SteLinear;

/// Training-mode Qwen3 attention block.
///
/// Uses SteLinear for projections (STE on 1-bit quantized weights)
/// and standard burn RmsNorm (with autodiff support).
#[derive(Module, Debug)]
pub struct Qwen3TrainBlock<B: Backend> {
    pub attn_norm: RmsNorm<B>,
    pub q_proj: SteLinear<B>,
    pub k_proj: SteLinear<B>,
    pub v_proj: SteLinear<B>,
    pub o_proj: SteLinear<B>,
    pub q_norm: RmsNorm<B>,
    pub k_norm: RmsNorm<B>,
    pub ffn_norm: RmsNorm<B>,
    pub ffn_gate: SteLinear<B>,
    pub ffn_up: SteLinear<B>,
    pub ffn_down: SteLinear<B>,
    pub n_q_heads: usize,
    pub n_kv_heads: usize,
    pub d_head: usize,
}

impl<B: Backend> Qwen3TrainBlock<B> {
    pub fn new(cfg: &ModelConfig, device: &B::Device) -> Self {
        let d = cfg.d_model;
        let q_dim = cfg.n_q_heads * cfg.d_head;
        let kv_dim = cfg.n_kv_heads * cfg.d_head;

        Qwen3TrainBlock {
            attn_norm: RmsNormConfig::new(d).with_epsilon(cfg.rms_norm_eps).init(device),
            q_proj: SteLinear::from_weight(Tensor::<B, 2>::zeros([q_dim, d], device)),
            k_proj: SteLinear::from_weight(Tensor::<B, 2>::zeros([kv_dim, d], device)),
            v_proj: SteLinear::from_weight(Tensor::<B, 2>::zeros([kv_dim, d], device)),
            o_proj: SteLinear::from_weight(Tensor::<B, 2>::zeros([d, q_dim], device)),
            q_norm: RmsNormConfig::new(cfg.d_head).with_epsilon(cfg.rms_norm_eps).init(device),
            k_norm: RmsNormConfig::new(cfg.d_head).with_epsilon(cfg.rms_norm_eps).init(device),
            ffn_norm: RmsNormConfig::new(d).with_epsilon(cfg.rms_norm_eps).init(device),
            ffn_gate: SteLinear::from_weight(Tensor::<B, 2>::zeros([cfg.d_ff, d], device)),
            ffn_up: SteLinear::from_weight(Tensor::<B, 2>::zeros([cfg.d_ff, d], device)),
            ffn_down: SteLinear::from_weight(Tensor::<B, 2>::zeros([d, cfg.d_ff], device)),
            n_q_heads: cfg.n_q_heads,
            n_kv_heads: cfg.n_kv_heads,
            d_head: cfg.d_head,
        }
    }

    pub fn forward(
        &self,
        x: Tensor<B, 3>,
        rope: &RotaryEncoding<B>,
        pos: usize,
    ) -> Tensor<B, 3> {
        let [batch, seq_len, _] = x.dims();

        // Self-attention with pre-norm
        let residual = x.clone();
        let h = self.attn_norm.forward(x);

        let q = self.q_proj.forward(h.clone());
        let k = self.k_proj.forward(h.clone());
        let v = self.v_proj.forward(h);

        // Reshape to heads
        let q = q.reshape([batch, seq_len, self.n_q_heads, self.d_head]);
        let k = k.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);
        let v = v.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);

        // Per-head QK normalization
        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        // RoPE (standard burn impl — works with autodiff)
        let q = rope.apply(q, pos);
        let k = rope.apply(k, pos);

        // Swap to [b, n, s, d_h]
        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        // GQA: repeat KV heads
        let (k, v) = if self.n_q_heads != self.n_kv_heads {
            let n_rep = self.n_q_heads / self.n_kv_heads;
            (repeat_kv(k, n_rep), repeat_kv(v, n_rep))
        } else {
            (k, v)
        };

        // Scaled dot-product attention (standard, autodiff-compatible)
        let scale = 1.0 / (self.d_head as f64).sqrt();
        let mut attn = q.matmul(k.transpose()).mul_scalar(scale);
        if seq_len > 1 {
            let device = attn.device();
            attn = attn + causal_mask::<B>(seq_len, pos, &device);
        }
        let attn = softmax(attn, 3);
        let out = attn.matmul(v);

        let out = out.swap_dims(1, 2).reshape([batch, seq_len, self.n_q_heads * self.d_head]);
        let h = self.o_proj.forward(out);
        let h = residual + h;

        // FFN with pre-norm (SwiGLU)
        let residual = h.clone();
        let h = self.ffn_norm.forward(h);
        let gate = silu(self.ffn_gate.forward(h.clone()));
        let up = self.ffn_up.forward(h);
        let h = self.ffn_down.forward(gate * up);
        residual + h
    }

    /// Forward pass for frozen layers — skips STE quantize_dequantize to save VRAM.
    /// Uses raw weight matmul and detaches output to prevent gradient tracking.
    pub fn forward_frozen(
        &self,
        x: Tensor<B, 3>,
        rope: &RotaryEncoding<B>,
        pos: usize,
    ) -> Tensor<B, 3> {
        let [batch, seq_len, _] = x.dims();

        let residual = x.clone();
        let h = self.attn_norm.forward(x);

        // Direct matmul with weights (no STE)
        let q = raw_linear(&self.q_proj.weight.val(), h.clone());
        let k = raw_linear(&self.k_proj.weight.val(), h.clone());
        let v = raw_linear(&self.v_proj.weight.val(), h);

        let q = q.reshape([batch, seq_len, self.n_q_heads, self.d_head]);
        let k = k.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);
        let v = v.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);

        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        let q = rope.apply(q, pos);
        let k = rope.apply(k, pos);

        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        let (k, v) = if self.n_q_heads != self.n_kv_heads {
            let n_rep = self.n_q_heads / self.n_kv_heads;
            (repeat_kv(k, n_rep), repeat_kv(v, n_rep))
        } else {
            (k, v)
        };

        let scale = 1.0 / (self.d_head as f64).sqrt();
        let mut attn = q.matmul(k.transpose()).mul_scalar(scale);
        if seq_len > 1 {
            let device = attn.device();
            attn = attn + causal_mask::<B>(seq_len, pos, &device);
        }
        let attn = softmax(attn, 3);
        let out = attn.matmul(v);

        let out = out.swap_dims(1, 2).reshape([batch, seq_len, self.n_q_heads * self.d_head]);
        let h = raw_linear(&self.o_proj.weight.val(), out);
        let h = residual + h;

        let residual = h.clone();
        let h = self.ffn_norm.forward(h);
        let gate = silu(raw_linear(&self.ffn_gate.weight.val(), h.clone()));
        let up = raw_linear(&self.ffn_up.weight.val(), h);
        let h = raw_linear(&self.ffn_down.weight.val(), gate * up);
        (residual + h).detach()
    }
}

/// Full training-mode Qwen3 model.
#[derive(Module, Debug)]
pub struct Qwen3TrainModel<B: Backend> {
    pub embedding: Embedding<B>,
    pub layers: Vec<Qwen3TrainBlock<B>>,
    pub final_norm: RmsNorm<B>,
    pub lm_head: Linear<B>,
    pub config: ModelConfig,
}

impl<B: Backend> Qwen3TrainModel<B> {
    pub fn new(cfg: &ModelConfig, device: &B::Device) -> Self {
        let layers = (0..cfg.n_layers)
            .map(|_| Qwen3TrainBlock::new(cfg, device))
            .collect();

        Qwen3TrainModel {
            embedding: EmbeddingConfig::new(cfg.vocab_size, cfg.d_model).init(device),
            layers,
            final_norm: RmsNormConfig::new(cfg.d_model).with_epsilon(cfg.rms_norm_eps).init(device),
            lm_head: LinearConfig::new(cfg.d_model, cfg.vocab_size).with_bias(false).init(device),
            config: cfg.clone(),
        }
    }

    /// Create the standard burn RotaryEncoding for this model's config.
    pub fn create_rope(&self, device: &B::Device) -> RotaryEncoding<B> {
        let cfg = &self.config;
        RotaryEncodingConfig::new(cfg.max_seq_len, cfg.d_head)
            .with_theta(cfg.rope_theta)
            .init(device)
        // Note: YARN scaling omitted for simplicity — fine-tuning on short
        // sequences (< original_ctx) doesn't need it. Add if needed.
    }

    /// Forward pass: token IDs → logits [batch, seq_len, vocab].
    ///
    /// `trainable_layers` controls how many layers (from the end) participate
    /// in gradient computation. Earlier layers run with `detach()` to prevent
    /// the autodiff graph from growing through them, saving significant VRAM.
    /// Set to `n_layers` to train all layers.
    pub fn forward(
        &self,
        tokens: Tensor<B, 2, Int>,
        rope: &RotaryEncoding<B>,
        trainable_layers: usize,
    ) -> Tensor<B, 3> {
        let mut h = self.embedding.forward(tokens).detach();
        let n = self.layers.len();
        let freeze_until = n.saturating_sub(trainable_layers);

        for (i, layer) in self.layers.iter().enumerate() {
            if i < freeze_until {
                // Frozen layer: skip STE quantize_dequantize to save VRAM.
                // Use raw weight matmul instead (no STE trick needed for frozen params).
                h = layer.forward_frozen(h, rope, 0);
            } else {
                h = layer.forward(h, rope, 0);
            }
        }

        let h = self.final_norm.forward(h);
        // Detach before lm_head to freeze it — saves ~1.8GB on the 151K vocab matrix
        let h = if trainable_layers < self.layers.len() {
            self.lm_head.forward(h.detach())
        } else {
            self.lm_head.forward(h)
        };
        h
    }

    /// Forward pass returning loss for training.
    ///
    /// tokens: [batch, seq_len] — the full sequence including target tokens.
    /// The loss is computed as cross-entropy between logits[:-1] and tokens[1:].
    /// `trainable_layers`: how many layers (from end) get gradients.
    pub fn forward_loss(
        &self,
        tokens: Tensor<B, 2, Int>,
        rope: &RotaryEncoding<B>,
        trainable_layers: usize,
    ) -> Tensor<B, 1> {
        let [batch, seq_len] = tokens.dims();

        let logits = self.forward(tokens.clone(), rope, trainable_layers);

        // Shift: predict next token from each position
        let logits = logits.slice([0..batch, 0..(seq_len - 1), 0..self.config.vocab_size]);
        let targets = tokens.slice([0..batch, 1..seq_len]);

        // Flatten for cross-entropy: [batch * (seq_len-1), vocab] vs [batch * (seq_len-1)]
        let logits_flat: Tensor<B, 2> = logits.reshape([batch * (seq_len - 1), self.config.vocab_size]);
        let targets_flat: Tensor<B, 1, Int> = targets.reshape([batch * (seq_len - 1)]);

        burn::nn::loss::CrossEntropyLossConfig::new()
            .init(&logits_flat.device())
            .forward(logits_flat, targets_flat)
    }
}

/// Simple weight × input matmul without STE or Module overhead.
fn raw_linear<B: Backend>(weight: &Tensor<B, 2>, input: Tensor<B, 3>) -> Tensor<B, 3> {
    let [batch, seq, _d_in] = input.dims();
    let [d_out, d_in] = weight.dims();
    let flat: Tensor<B, 2> = input.reshape([batch * seq, d_in]);
    let out = flat.matmul(weight.clone().transpose());
    out.reshape([batch, seq, d_out])
}

/// Repeat KV heads to match Q head count for GQA.
fn repeat_kv<B: Backend>(x: Tensor<B, 4>, n_rep: usize) -> Tensor<B, 4> {
    if n_rep == 1 {
        return x;
    }
    let [b, h, s, d] = x.dims();
    x.reshape([b, h, 1, s, d])
        .expand([b, h, n_rep, s, d])
        .reshape([b, h * n_rep, s, d])
}

/// Build a causal attention mask (additive, -inf for masked positions).
fn causal_mask<B: Backend>(seq_len: usize, past_len: usize, device: &B::Device) -> Tensor<B, 4> {
    let total_len = seq_len + past_len;
    let mask_val = -1e4_f32;

    let rows = Tensor::<B, 1, Int>::arange(0..seq_len as i64, device)
        .reshape([seq_len, 1])
        .float();
    let cols = Tensor::<B, 1, Int>::arange(0..total_len as i64, device)
        .reshape([1, total_len])
        .float();

    let allowed = cols.lower_equal(rows + past_len as f32);
    let mask = allowed
        .float()
        .mul_scalar(-1.0)
        .add_scalar(1.0)
        .mul_scalar(mask_val);

    mask.unsqueeze_dim::<3>(0).unsqueeze_dim::<4>(0)
}
