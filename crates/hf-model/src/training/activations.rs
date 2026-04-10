use burn::tensor::Tensor;
use crate::B;

/// Per-layer activations stored during the training forward pass.
/// All tensors are tiny at small batch/seq (e.g., ~1.5 MB per layer at batch=1, seq=11).
pub struct LayerActivations {
    /// Layer input [batch, seq, d_model] — for residual + norm backward.
    pub x: Tensor<B, 3>,
    /// Post-attn-norm output [batch, seq, d_model] — input to Q/K/V projections.
    pub h_normed_attn: Tensor<B, 3>,
    /// Q after reshape, before QK norm [batch, seq, n_q_heads, d_head].
    pub q_pre_norm: Tensor<B, 4>,
    /// K after reshape, before QK norm [batch, seq, n_kv_heads, d_head].
    pub k_pre_norm: Tensor<B, 4>,
    /// Q after norm+RoPE+swap [batch, n_q_heads, seq, d_head].
    pub q: Tensor<B, 4>,
    /// K after norm+RoPE+swap [batch, n_kv_heads, seq, d_head] (pre-GQA expand).
    pub k: Tensor<B, 4>,
    /// V after reshape+swap [batch, n_kv_heads, seq, d_head] (pre-GQA expand).
    pub v: Tensor<B, 4>,
    /// Softmax attention weights [batch, n_q_heads, seq, seq].
    pub attn_weights: Tensor<B, 4>,
    /// Input to O projection [batch, seq, n_q_heads * d_head].
    pub o_proj_input: Tensor<B, 3>,
    /// Attention sublayer output (after residual add) [batch, seq, d_model].
    pub h_after_attn: Tensor<B, 3>,
    /// Post-ffn-norm output [batch, seq, d_model] — input to gate/up projections.
    pub h_normed_ffn: Tensor<B, 3>,
    /// Gate projection output before SiLU [batch, seq, d_ff] — for SiLU backward.
    pub gate_pre_silu: Tensor<B, 3>,
    /// Up projection output [batch, seq, d_ff] — for gate*up backward.
    pub up_val: Tensor<B, 3>,
}

/// All activations from a full model forward pass.
pub struct ForwardActivations {
    /// Embedding output [batch, seq, d_model].
    pub embedding_output: Tensor<B, 3>,
    /// Per-layer activations.
    pub layers: Vec<LayerActivations>,
    /// Input to final_norm (= last layer output) [batch, seq, d_model].
    pub final_norm_input: Tensor<B, 3>,
    /// Output of final_norm (= input to lm_head) [batch, seq, d_model].
    pub final_norm_output: Tensor<B, 3>,
}
