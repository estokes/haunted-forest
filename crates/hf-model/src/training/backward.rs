use burn::tensor::{Int, Tensor, IndexingUpdateOp};

use crate::B;
use crate::bit_linear::BitLinear;
use crate::fused_rope::FusedRoPE;
use crate::fused_rms_norm::FusedRmsNorm;
use crate::qwen3::{ModelConfig, Qwen3Block, Qwen3Model};
use super::activations::{ForwardActivations, LayerActivations};

/// Weight gradients for all projections in one layer.
pub struct LayerGradients {
    pub q_proj: Tensor<B, 2>,
    pub k_proj: Tensor<B, 2>,
    pub v_proj: Tensor<B, 2>,
    pub o_proj: Tensor<B, 2>,
    pub ffn_gate: Tensor<B, 2>,
    pub ffn_up: Tensor<B, 2>,
    pub ffn_down: Tensor<B, 2>,
}

/// Weight gradients for all layers in the model.
pub struct ModelGradients {
    pub layers: Vec<LayerGradients>,
    pub lm_head: Tensor<B, 2>,
}

// ---- Top-level backward ----

/// Compute cross-entropy loss and its gradient w.r.t. logits.
///
/// logits: [batch, seq_len, vocab_size]
/// targets: token IDs [seq_len] (will be shifted: predict position i+1 from position i)
///
/// Returns (scalar_loss, grad_logits [batch*(seq_len-1), vocab_size]).
pub fn cross_entropy_loss_and_grad(
    logits: Tensor<B, 3>,
    target_ids: &[u32],
    vocab_size: usize,
) -> (f32, Tensor<B, 2>) {
    let [batch, seq_len, _vocab] = logits.dims();
    let device = logits.device();
    let n = batch * (seq_len - 1);

    // Shift: logits[:-1] predict targets[1:]
    let logits_shifted: Tensor<B, 2> = logits
        .slice([0..batch, 0..(seq_len - 1), 0..vocab_size])
        .reshape([n, vocab_size]);

    let targets = Tensor::<B, 1, Int>::from_data(
        burn::tensor::TensorData::new(
            target_ids[1..].iter().map(|&t| t as i64).collect::<Vec<_>>(),
            [target_ids.len() - 1],
        ),
        &device,
    );

    // Softmax
    let probs = burn::tensor::activation::softmax(logits_shifted, 1);

    // Loss: -log(prob at target)
    let target_2d = targets.clone().reshape([n, 1]);
    let target_probs = probs.clone().gather(1, target_2d);
    let loss = target_probs.log().neg().mean().into_scalar();

    // Gradient: (softmax - one_hot) / n
    let one_hot = Tensor::<B, 2>::zeros([n, vocab_size], &device)
        .scatter(1, targets.reshape([n, 1]), Tensor::ones([n, 1], &device), IndexingUpdateOp::Add);
    let grad = (probs - one_hot).div_scalar(n as f32);

    (loss, grad)
}

/// Backward through lm_head + final_norm, returning the gradient
/// ready to flow into the layer stack and the lm_head weight gradient.
pub fn backward_head_and_norm(
    grad_logits: Tensor<B, 2>,
    model: &Qwen3Model,
    acts: &ForwardActivations,
    cfg: &ModelConfig,
) -> (Tensor<B, 3>, Tensor<B, 2>) {
    let [batch, seq_len] = [acts.embedding_output.dims()[0], acts.embedding_output.dims()[1]];

    let device = grad_logits.device();
    let grad_3d: Tensor<B, 3> = grad_logits.reshape([batch, seq_len - 1, cfg.vocab_size]);
    let zero_pad: Tensor<B, 3> = Tensor::zeros([batch, 1, cfg.vocab_size], &device);
    let grad_logits_full = Tensor::cat(vec![grad_3d, zero_pad], 1);

    // Backward through lm_head
    let grad_lm_2d: Tensor<B, 2> = grad_logits_full.reshape([batch * seq_len, cfg.vocab_size]);
    let fn_out_2d: Tensor<B, 2> = acts.final_norm_output.clone().reshape([batch * seq_len, cfg.d_model]);
    let (grad_w_lm, grad_fn_out) = linear_backward_lm_head(
        grad_lm_2d, fn_out_2d, &model.lm_head,
    );
    let grad: Tensor<B, 3> = grad_fn_out.reshape([batch, seq_len, cfg.d_model]);

    // Backward through final_norm
    let grad_2d: Tensor<B, 2> = grad.reshape([batch * seq_len, cfg.d_model]);
    let fn_input_2d: Tensor<B, 2> = acts.final_norm_input.clone().reshape([batch * seq_len, cfg.d_model]);
    let grad_2d = rmsnorm_backward(grad_2d, fn_input_2d, &model.final_norm.gamma, model.final_norm.epsilon);
    let grad = grad_2d.reshape([batch, seq_len, cfg.d_model]);

    (grad, grad_w_lm)
}

/// Backward through one Qwen3Block layer.
/// Public so the trainer can interleave backward + optimizer per-layer.
pub fn backward_one_layer(
    grad_output: Tensor<B, 3>,
    acts: &LayerActivations,
    block: &Qwen3Block,
    rope: &FusedRoPE,
    cfg: &ModelConfig,
) -> (LayerGradients, Tensor<B, 3>) {
    layer_backward(grad_output, acts, block, rope, cfg)
}

// ---- Layer backward ----

/// Backward through one Qwen3Block.
/// Returns (weight gradients, input gradient [batch, seq, d_model]).
fn layer_backward(
    grad_output: Tensor<B, 3>,
    acts: &LayerActivations,
    block: &Qwen3Block,
    rope: &FusedRoPE,
    cfg: &ModelConfig,
) -> (LayerGradients, Tensor<B, 3>) {
    let [batch, seq_len, d_model] = grad_output.dims();
    let bs = batch * seq_len;

    // ---- FFN backward ----
    // output = residual + ffn_down(silu(gate) * up)
    // Both residual and ffn path get the full gradient
    let grad_ffn_out = grad_output.clone(); // gradient into ffn_down output
    let grad_residual_ffn = grad_output;    // gradient into the residual path

    // Backward through ffn_down: Y = X @ W.T
    let silu_gate = burn::tensor::activation::silu(acts.gate_pre_silu.clone());
    let ffn_down_input: Tensor<B, 2> = (silu_gate.clone() * acts.up_val.clone()).reshape([bs, cfg.d_ff]);
    let grad_ffn_2d: Tensor<B, 2> = grad_ffn_out.reshape([bs, d_model]);
    let (grad_w_down, grad_down_input) = linear_backward(
        grad_ffn_2d, ffn_down_input, &block.ffn_down,
    );
    let grad_down_input: Tensor<B, 3> = grad_down_input.reshape([batch, seq_len, cfg.d_ff]);

    // Backward through SwiGLU: y = silu(gate) * up
    let grad_silu_gate = grad_down_input.clone() * acts.up_val.clone();
    let grad_up = grad_down_input * silu_gate;

    // Backward through SiLU
    let grad_gate = silu_backward(grad_silu_gate, acts.gate_pre_silu.clone());

    // Backward through ffn_gate and ffn_up projections
    let h_ffn_2d: Tensor<B, 2> = acts.h_normed_ffn.clone().reshape([bs, d_model]);
    let grad_gate_2d: Tensor<B, 2> = grad_gate.reshape([bs, cfg.d_ff]);
    let grad_up_2d: Tensor<B, 2> = grad_up.reshape([bs, cfg.d_ff]);
    let (grad_w_gate, grad_h_from_gate) = linear_backward(grad_gate_2d, h_ffn_2d.clone(), &block.ffn_gate);
    let (grad_w_up, grad_h_from_up) = linear_backward(grad_up_2d, h_ffn_2d, &block.ffn_up);
    let grad_h_normed_ffn: Tensor<B, 2> = grad_h_from_gate + grad_h_from_up;

    // Backward through ffn_norm
    let h_after_attn_2d: Tensor<B, 2> = acts.h_after_attn.clone().reshape([bs, d_model]);
    let grad_h_after_attn_2d = rmsnorm_backward(
        grad_h_normed_ffn, h_after_attn_2d, &block.ffn_norm.gamma, block.ffn_norm.epsilon,
    );

    // Add FFN residual gradient
    let grad_h_after_attn: Tensor<B, 3> = grad_h_after_attn_2d.reshape([batch, seq_len, d_model]);
    let grad_h_after_attn = grad_h_after_attn + grad_residual_ffn;

    // ---- Attention backward ----
    // h_after_attn = residual + o_proj(attn_out)
    let grad_attn_out = grad_h_after_attn.clone();
    let grad_residual_attn = grad_h_after_attn;

    // Backward through o_proj
    let o_input_2d: Tensor<B, 2> = acts.o_proj_input.clone().reshape([bs, cfg.n_q_heads * cfg.d_head]);
    let grad_o_2d: Tensor<B, 2> = grad_attn_out.reshape([bs, d_model]);
    let (grad_w_o, grad_o_input) = linear_backward(grad_o_2d, o_input_2d, &block.attn.o_proj);

    // Reshape to [b, n_q, s, d_h] (reverse of forward's swap+reshape)
    let grad_attn_4d = grad_o_input
        .reshape([batch, seq_len, cfg.n_q_heads, cfg.d_head])
        .swap_dims(1, 2); // [b, n_q, s, d_h]

    // Backward through attn_out = attn_weights @ V_expanded
    let n_rep = cfg.n_q_heads / cfg.n_kv_heads;
    let v_expanded = if n_rep > 1 {
        super::forward::repeat_kv(acts.v.clone(), n_rep)
    } else {
        acts.v.clone()
    };

    // dL/d(attn_weights) = dL/d(out) @ V.T
    let grad_attn_weights = grad_attn_4d.clone().matmul(v_expanded.clone().transpose());
    // dL/d(V_expanded) = attn_weights.T @ dL/d(out)
    let grad_v_expanded = acts.attn_weights.clone().transpose().matmul(grad_attn_4d);

    // Backward through softmax
    let grad_scores = softmax_backward(grad_attn_weights, acts.attn_weights.clone());

    // Backward through scale
    let scale = 1.0 / (cfg.d_head as f64).sqrt();
    let grad_scores = grad_scores.mul_scalar(scale);

    // Backward through Q @ K_expanded.T
    let k_expanded = if n_rep > 1 {
        super::forward::repeat_kv(acts.k.clone(), n_rep)
    } else {
        acts.k.clone()
    };
    let grad_q = grad_scores.clone().matmul(k_expanded); // [b, n_q, s, d_h]
    let grad_k_expanded = grad_scores.transpose().matmul(acts.q.clone()); // [b, n_q, s, d_h]

    // GQA backward: sum over repeated heads
    let grad_k = gqa_repeat_backward(grad_k_expanded, cfg.n_kv_heads, n_rep);
    let grad_v = gqa_repeat_backward(grad_v_expanded, cfg.n_kv_heads, n_rep);

    // Swap back: [b, n, s, d_h] → [b, s, n, d_h]
    let grad_q = grad_q.swap_dims(1, 2);
    let grad_k = grad_k.swap_dims(1, 2);
    let grad_v = grad_v.swap_dims(1, 2);

    // Backward through RoPE (inverse rotation)
    let grad_q = rope.apply_inverse(grad_q, 0);
    let grad_k = rope.apply_inverse(grad_k, 0);

    // Backward through QK norms
    let [_, _, n_q, d_h] = acts.q_pre_norm.dims();
    let [_, _, n_kv, _] = acts.k_pre_norm.dims();
    let grad_q_2d: Tensor<B, 2> = grad_q.reshape([bs * n_q, d_h]);
    let q_pre_2d: Tensor<B, 2> = acts.q_pre_norm.clone().reshape([bs * n_q, d_h]);
    let grad_q_2d = rmsnorm_backward(grad_q_2d, q_pre_2d, &block.attn.q_norm.gamma, block.attn.q_norm.epsilon);
    let grad_k_2d: Tensor<B, 2> = grad_k.reshape([bs * n_kv, d_h]);
    let k_pre_2d: Tensor<B, 2> = acts.k_pre_norm.clone().reshape([bs * n_kv, d_h]);
    let grad_k_2d = rmsnorm_backward(grad_k_2d, k_pre_2d, &block.attn.k_norm.gamma, block.attn.k_norm.epsilon);

    // Reshape back to [b, s, n*d_h] for projection backward
    let grad_q_flat: Tensor<B, 2> = grad_q_2d.reshape([bs, n_q * d_h]);
    let grad_k_flat: Tensor<B, 2> = grad_k_2d.reshape([bs, n_kv * d_h]);
    let grad_v_flat: Tensor<B, 2> = grad_v.reshape([bs, n_kv * d_h]);

    // Backward through Q, K, V projections
    let h_attn_2d: Tensor<B, 2> = acts.h_normed_attn.clone().reshape([bs, d_model]);
    let (grad_w_q, grad_h_from_q) = linear_backward(grad_q_flat, h_attn_2d.clone(), &block.attn.q_proj);
    let (grad_w_k, grad_h_from_k) = linear_backward(grad_k_flat, h_attn_2d.clone(), &block.attn.k_proj);
    let (grad_w_v, grad_h_from_v) = linear_backward(grad_v_flat, h_attn_2d, &block.attn.v_proj);
    let grad_h_normed_attn: Tensor<B, 2> = grad_h_from_q + grad_h_from_k + grad_h_from_v;

    // Backward through attn_norm
    let x_2d: Tensor<B, 2> = acts.x.clone().reshape([bs, d_model]);
    let grad_x_2d = rmsnorm_backward(
        grad_h_normed_attn, x_2d, &block.attn_norm.gamma, block.attn_norm.epsilon,
    );

    // Add attention residual gradient
    let grad_x: Tensor<B, 3> = grad_x_2d.reshape([batch, seq_len, d_model]);
    let grad_input = grad_x + grad_residual_attn;

    let grads = LayerGradients {
        q_proj: grad_w_q,
        k_proj: grad_w_k,
        v_proj: grad_w_v,
        o_proj: grad_w_o,
        ffn_gate: grad_w_gate,
        ffn_up: grad_w_up,
        ffn_down: grad_w_down,
    };

    (grads, grad_input)
}

// ---- Primitive backward functions ----

/// Backward through linear: Y = X @ W.T where W is packed 1-bit.
fn linear_backward(
    grad_output: Tensor<B, 2>,
    input: Tensor<B, 2>,
    packed_weight: &BitLinear,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    // dW = dY.T @ X
    let grad_weight = grad_output.clone().transpose().matmul(input);

    // dX = dY @ W (GPU-resident dequant to scratch, no CPU roundtrip)
    let w = packed_weight.dequantize_weights_gpu(); // [d_out, d_in], ~23 MB scratch
    let grad_input = grad_output.matmul(w);
    // w dropped here, freeing scratch

    (grad_weight, grad_input)
}

/// Backward through lm_head (handles both BitLinear and Linear variants).
fn linear_backward_lm_head(
    grad_output: Tensor<B, 2>,
    input: Tensor<B, 2>,
    lm_head: &crate::qwen3::LmHead,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    let grad_weight = grad_output.clone().transpose().matmul(input.clone());

    let grad_input = match lm_head {
        crate::qwen3::LmHead::BitLinear(bl) => {
            let w = bl.dequantize_weights_gpu();
            grad_output.matmul(w)
        }
        crate::qwen3::LmHead::Linear(lin) => {
            // burn's Linear weight is [d_in, d_out], forward does X @ W
            // so backward: dX = dY @ W.T
            let w = lin.weight.val(); // [d_in, d_out]
            grad_output.matmul(w.transpose())
        }
    };

    (grad_weight, grad_input)
}

/// Backward through RmsNorm: y = x * gamma / rms(x).
fn rmsnorm_backward(
    grad_output: Tensor<B, 2>,
    x: Tensor<B, 2>,
    gamma: &Tensor<B, 1>,
    epsilon: f64,
) -> Tensor<B, 2> {
    let [_n, d] = x.dims();

    let x_sq_mean = x.clone().powf_scalar(2.0).mean_dim(1); // [n, 1]
    let rms = (x_sq_mean + epsilon).sqrt();
    let inv_rms = rms.recip(); // [n, 1]

    let gamma_exp = gamma.clone().unsqueeze_dim::<2>(0); // [1, d]
    let c = (x.clone() * gamma_exp.clone() * grad_output.clone())
        .sum_dim(1)
        .div_scalar(d as f32); // [n, 1]

    let inv_rms_sq = inv_rms.clone() * inv_rms.clone();
    inv_rms * (gamma_exp * grad_output - x * inv_rms_sq * c)
}

/// Backward through softmax: y = softmax(x).
fn softmax_backward(
    grad_output: Tensor<B, 4>,
    softmax_output: Tensor<B, 4>,
) -> Tensor<B, 4> {
    let y_dy = softmax_output.clone() * grad_output.clone();
    let s = y_dy.sum_dim(3); // sum over last dim
    softmax_output * (grad_output - s)
}

/// Backward through SiLU: y = x * sigmoid(x).
fn silu_backward(
    grad_output: Tensor<B, 3>,
    x: Tensor<B, 3>,
) -> Tensor<B, 3> {
    let sig = burn::tensor::activation::sigmoid(x.clone());
    let dsilu = sig.clone() * (x * (sig.mul_scalar(-1.0).add_scalar(1.0)) + 1.0);
    grad_output * dsilu
}

/// Backward through GQA repeat: sum gradient over repeated heads.
fn gqa_repeat_backward(
    grad_expanded: Tensor<B, 4>,
    n_kv_heads: usize,
    n_rep: usize,
) -> Tensor<B, 4> {
    if n_rep == 1 {
        return grad_expanded;
    }
    let [b, _n_q, s, d] = grad_expanded.dims();
    grad_expanded
        .reshape([b, n_kv_heads, n_rep, s, d])
        .sum_dim(2)
        .reshape([b, n_kv_heads, s, d])
}
