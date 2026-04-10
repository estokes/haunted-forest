use burn::tensor::{Int, Tensor};
use burn::tensor::activation::softmax;

use crate::B;
use crate::qwen3::{ModelConfig, Qwen3Block, Qwen3Model};
use crate::fused_rope::FusedRoPE;
use super::activations::{ForwardActivations, LayerActivations};

/// Run the full model forward pass, storing activations for backward.
///
/// Differs from inference forward:
/// - No KV cache (full sequence processed at once)
/// - No flash attention (explicit matmul so we can store attn_weights)
/// - Stores all intermediates needed for backward
pub fn forward_with_activations(
    model: &Qwen3Model,
    tokens: Tensor<B, 2, Int>,
    rope: &FusedRoPE,
) -> (Tensor<B, 3>, ForwardActivations) {
    let embedding_output = model.embedding.forward(tokens);
    let mut h = embedding_output.clone();
    let mut layer_acts = Vec::with_capacity(model.layers.len());

    for layer in &model.layers {
        let (h_next, acts) = block_forward_with_activations(layer, h, rope, &model.config);
        layer_acts.push(acts);
        h = h_next;
    }

    let final_norm_input = h.clone();
    let final_norm_output = model.final_norm.forward(h);
    let logits = model.lm_head.forward(final_norm_output.clone());

    let acts = ForwardActivations {
        embedding_output,
        layers: layer_acts,
        final_norm_input,
        final_norm_output,
    };

    (logits, acts)
}

/// Forward one Qwen3Block, storing activations for backward.
///
/// Uses fused GEMV for projections, fused RmsNorm, fused RoPE,
/// but explicit matmul for attention (not flash) so we store attn_weights.
fn block_forward_with_activations(
    block: &Qwen3Block,
    x: Tensor<B, 3>,
    rope: &FusedRoPE,
    cfg: &ModelConfig,
) -> (Tensor<B, 3>, LayerActivations) {
    let [batch, seq_len, _] = x.dims();

    // ---- Attention sublayer ----
    let residual = x.clone();
    let h_normed_attn = block.attn_norm.forward(x.clone());

    // Project Q, K, V via fused GEMV (reads packed weights directly)
    let q = block.attn.q_proj.forward(h_normed_attn.clone());
    let k = block.attn.k_proj.forward(h_normed_attn.clone());
    let v = block.attn.v_proj.forward(h_normed_attn.clone());

    // Reshape to heads [b, s, n, d_h]
    let q_pre_norm = q.reshape([batch, seq_len, cfg.n_q_heads, cfg.d_head]);
    let k_pre_norm = k.reshape([batch, seq_len, cfg.n_kv_heads, cfg.d_head]);
    let v = v.reshape([batch, seq_len, cfg.n_kv_heads, cfg.d_head]);

    // Per-head QK normalization
    let q = block.attn.q_norm.forward(q_pre_norm.clone());
    let k = block.attn.k_norm.forward(k_pre_norm.clone());

    // Apply fused RoPE (single kernel per call)
    let q = rope.apply(q, 0);
    let k = rope.apply(k, 0);

    // Swap to [b, n, s, d_h] for attention
    let q = q.swap_dims(1, 2);
    let k = k.swap_dims(1, 2);
    let v = v.swap_dims(1, 2);

    // GQA: repeat KV heads to match Q heads
    let (k_expanded, v_expanded) = if cfg.n_q_heads != cfg.n_kv_heads {
        let n_rep = cfg.n_q_heads / cfg.n_kv_heads;
        (repeat_kv(k.clone(), n_rep), repeat_kv(v.clone(), n_rep))
    } else {
        (k.clone(), v.clone())
    };

    // Explicit attention (not flash) — we need attn_weights for backward
    let scale = 1.0 / (cfg.d_head as f64).sqrt();
    let mut scores = q.clone().matmul(k_expanded.transpose()).mul_scalar(scale);

    // Causal mask
    if seq_len > 1 {
        let device = scores.device();
        scores = scores + causal_mask(seq_len, &device);
    }

    let attn_weights = softmax(scores, 3);
    let attn_out = attn_weights.clone().matmul(v_expanded);

    // Reshape back: [b, n_q, s, d_h] → [b, s, n_q * d_h]
    let o_proj_input = attn_out.clone()
        .swap_dims(1, 2)
        .reshape([batch, seq_len, cfg.n_q_heads * cfg.d_head]);

    let o = block.attn.o_proj.forward(o_proj_input.clone());
    let h_after_attn = residual + o;

    // ---- FFN sublayer (SwiGLU) ----
    let residual = h_after_attn.clone();
    let h_normed_ffn = block.ffn_norm.forward(h_after_attn.clone());

    let gate_pre_silu = block.ffn_gate.forward(h_normed_ffn.clone());
    let up_val = block.ffn_up.forward(h_normed_ffn.clone());

    let gate_activated = burn::tensor::activation::silu(gate_pre_silu.clone());
    let ffn_input = gate_activated * up_val.clone();
    let down = block.ffn_down.forward(ffn_input);
    let output = residual + down;

    let acts = LayerActivations {
        x,
        h_normed_attn,
        q_pre_norm,
        k_pre_norm,
        q,
        k, // pre-GQA expand
        v, // pre-GQA expand
        attn_weights,
        o_proj_input,
        h_after_attn,
        h_normed_ffn,
        gate_pre_silu,
        up_val,
    };

    (output, acts)
}

/// Causal attention mask (additive, -inf for masked positions).
fn causal_mask(seq_len: usize, device: &burn::backend::cuda::CudaDevice) -> Tensor<B, 4> {
    let mask_val = -1e4_f32;
    let rows = Tensor::<B, 1, Int>::arange(0..seq_len as i64, device)
        .reshape([seq_len, 1])
        .float();
    let cols = Tensor::<B, 1, Int>::arange(0..seq_len as i64, device)
        .reshape([1, seq_len])
        .float();
    let allowed = cols.lower_equal(rows);
    let mask = allowed.float().mul_scalar(-1.0).add_scalar(1.0).mul_scalar(mask_val);
    mask.unsqueeze_dim::<3>(0).unsqueeze_dim::<4>(0)
}

/// Repeat KV heads to match Q head count for GQA.
pub(crate) fn repeat_kv(x: Tensor<B, 4>, n_rep: usize) -> Tensor<B, 4> {
    if n_rep == 1 {
        return x;
    }
    let [b, h, s, d] = x.dims();
    x.reshape([b, h, 1, s, d])
        .expand([b, h, n_rep, s, d])
        .reshape([b, h * n_rep, s, d])
}
