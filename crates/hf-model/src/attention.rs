use burn::nn::RotaryEncoding;
use burn::tensor::Tensor;

use crate::B;
use crate::bit_linear::BitLinear;
use crate::fused_rms_norm::FusedRmsNorm;

/// Grouped Query Attention with per-head QK normalization.
///
/// Qwen3's attention differs from standard GQA in that it applies RMSNorm
/// to Q and K after projection and head reshaping, but before RoPE.
///
/// Projection weights are stored in packed Q1_0_g128 format (BitLinear)
/// and dequantized on-the-fly during each forward pass.
#[derive(Debug)]
pub struct Attention {
    pub q_proj: BitLinear,
    pub k_proj: BitLinear,
    pub v_proj: BitLinear,
    pub o_proj: BitLinear,
    pub q_norm: FusedRmsNorm,
    pub k_norm: FusedRmsNorm,
    pub n_q_heads: usize,
    pub n_kv_heads: usize,
    pub d_head: usize,
    pub scale: f64,
}

/// Configuration for constructing an Attention module (used during weight loading).
pub struct AttentionConfig {
    pub d_model: usize,
    pub n_q_heads: usize,
    pub n_kv_heads: usize,
    pub d_head: usize,
    pub eps: f64,
}

impl AttentionConfig {
    /// Initialize attention with placeholder BitLinear (will be replaced by loader).
    pub fn init(&self, device: &burn::backend::cuda::CudaDevice) -> Attention {
        Attention {
            q_proj: BitLinear::empty(device),
            k_proj: BitLinear::empty(device),
            v_proj: BitLinear::empty(device),
            o_proj: BitLinear::empty(device),
            q_norm: FusedRmsNorm::empty(self.d_head, self.eps, device),
            k_norm: FusedRmsNorm::empty(self.d_head, self.eps, device),
            n_q_heads: self.n_q_heads,
            n_kv_heads: self.n_kv_heads,
            d_head: self.d_head,
            scale: 1.0 / (self.d_head as f64).sqrt(),
        }
    }
}

/// Per-layer KV cache for autoregressive generation.
pub struct KvCache {
    pub k: Option<Tensor<B, 4>>, // [batch, n_kv_heads, cached_seq, d_head]
    pub v: Option<Tensor<B, 4>>, // [batch, n_kv_heads, cached_seq, d_head]
}

impl KvCache {
    pub fn empty() -> Self {
        KvCache { k: None, v: None }
    }
}

impl Attention {
    /// Forward pass.
    ///
    /// # Arguments
    /// - `x`: [batch, seq_len, d_model]
    /// - `rope`: rotary position encoding module
    /// - `pos`: starting position in sequence (for KV cache / RoPE offset)
    /// - `mask`: optional causal mask [batch, 1, seq_len, total_seq_len] (additive, -inf for masked)
    /// - `cache`: optional KV cache (mutated in-place)
    ///
    /// # Returns
    /// Output tensor [batch, seq_len, d_model]
    pub fn forward(
        &self,
        x: Tensor<B, 3>,
        rope: &RotaryEncoding<B>,
        pos: usize,
        mask: Option<Tensor<B, 4>>,
        cache: &mut KvCache,
    ) -> Tensor<B, 3> {
        let [batch, seq_len, _] = x.dims();

        // Project Q, K, V
        let q = self.q_proj.forward(x.clone());
        let k = self.k_proj.forward(x.clone());
        let v = self.v_proj.forward(x);

        // Reshape to heads: [b, s, n, d_h] → [b, n, s, d_h]
        let q = q
            .reshape([batch, seq_len, self.n_q_heads, self.d_head])
            .swap_dims(1, 2);
        let k = k
            .reshape([batch, seq_len, self.n_kv_heads, self.d_head])
            .swap_dims(1, 2);
        let v = v
            .reshape([batch, seq_len, self.n_kv_heads, self.d_head])
            .swap_dims(1, 2);

        // Per-head QK normalization (Qwen3-specific)
        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        // Apply RoPE with position offset
        let q = rope.apply(q, pos);
        let k = rope.apply(k, pos);

        // Update KV cache
        let (k, v) = if let (Some(cached_k), Some(cached_v)) = (cache.k.take(), cache.v.take()) {
            let k = Tensor::cat(vec![cached_k, k], 2);
            let v = Tensor::cat(vec![cached_v, v], 2);
            (k, v)
        } else {
            (k, v)
        };
        cache.k = Some(k.clone());
        cache.v = Some(v.clone());

        // GQA: repeat KV heads to match Q heads
        let (k, v) = if self.n_q_heads != self.n_kv_heads {
            let n_rep = self.n_q_heads / self.n_kv_heads;
            (repeat_kv(k, n_rep), repeat_kv(v, n_rep))
        } else {
            (k, v)
        };

        // Scaled dot-product attention
        let attn_weights = q.matmul(k.transpose())
            .mul_scalar(self.scale);

        let attn_weights = if let Some(mask) = mask {
            attn_weights + mask
        } else {
            attn_weights
        };

        let attn_weights = burn::tensor::activation::softmax(attn_weights, 3);

        let out = attn_weights.matmul(v);

        // Reshape back: [b, n_q, s_q, d_h] → [b, s_q, n_q * d_h]
        let out = out
            .swap_dims(1, 2)
            .reshape([batch, seq_len, self.n_q_heads * self.d_head]);

        // Output projection
        self.o_proj.forward(out)
    }
}

/// Repeat KV heads to match Q head count for GQA.
fn repeat_kv(x: Tensor<B, 4>, n_rep: usize) -> Tensor<B, 4> {
    if n_rep == 1 {
        return x;
    }
    let [b, h, s, d] = x.dims();
    x.reshape([b, h, 1, s, d])
        .expand([b, h, n_rep, s, d])
        .reshape([b, h * n_rep, s, d])
}
