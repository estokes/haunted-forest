use burn::tensor::Tensor;
use burn::tensor::module::attention;
use burn_backend::ops::AttentionModuleOptions;

use crate::B;
use crate::bit_linear::BitLinear;
use crate::fused_rope::FusedRoPE;
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
        }
    }
}

/// Pre-allocated KV cache for autoregressive generation.
///
/// Allocates to max_seq_len upfront; new KV is written at the current
/// position via slice_assign, avoiding per-token allocation and full-copy.
/// The Option wrapper allows moving the tensor out to get exclusive ownership,
/// which lets slice_assign operate in-place (refcount = 1).
pub struct KvCache {
    pub k: Option<Tensor<B, 4>>, // [batch, n_kv_heads, max_seq_len, d_head]
    pub v: Option<Tensor<B, 4>>,
    pub len: usize,
}

impl KvCache {
    pub fn new(
        batch: usize,
        n_kv_heads: usize,
        max_seq_len: usize,
        d_head: usize,
        device: &burn::backend::cuda::CudaDevice,
    ) -> Self {
        KvCache {
            k: Some(Tensor::zeros([batch, n_kv_heads, max_seq_len, d_head], device)),
            v: Some(Tensor::zeros([batch, n_kv_heads, max_seq_len, d_head], device)),
            len: 0,
        }
    }
}

impl Attention {
    /// Forward pass.
    ///
    /// # Arguments
    /// - `x`: [batch, seq_len, d_model]
    /// - `rope`: fused rotary position encoding
    /// - `pos`: starting position in sequence (for KV cache / RoPE offset)
    /// - `is_causal`: whether to apply causal masking (true during prefill, false during decode)
    /// - `cache`: KV cache (mutated in-place)
    ///
    /// # Returns
    /// Output tensor [batch, seq_len, d_model]
    pub fn forward(
        &self,
        x: Tensor<B, 3>,
        rope: &FusedRoPE,
        pos: usize,
        is_causal: bool,
        cache: &mut KvCache,
    ) -> Tensor<B, 3> {
        let [batch, seq_len, _] = x.dims();

        // Project Q, K, V
        let q = self.q_proj.forward(x.clone());
        let k = self.k_proj.forward(x.clone());
        let v = self.v_proj.forward(x);

        // Reshape to heads: [b, s, n, d_h] — d_head is contiguous
        let q = q.reshape([batch, seq_len, self.n_q_heads, self.d_head]);
        let k = k.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);
        let v = v.reshape([batch, seq_len, self.n_kv_heads, self.d_head]);

        // Per-head QK normalization (operates on last dim = d_head)
        let q = self.q_norm.forward(q);
        let k = self.k_norm.forward(k);

        // Apply fused RoPE (single kernel per call, operates on contiguous d_head pairs)
        let q = rope.apply(q, pos);
        let k = rope.apply(k, pos);

        // Now swap to [b, n, s, d_h] for attention computation
        let q = q.swap_dims(1, 2);
        let k = k.swap_dims(1, 2);
        let v = v.swap_dims(1, 2);

        // Move buffer out to get exclusive ownership (refcount=1), enabling in-place write
        let pos_end = pos + seq_len;
        let k_buf = cache.k.take().unwrap().slice_assign(
            [0..batch, 0..self.n_kv_heads, pos..pos_end, 0..self.d_head],
            k,
        );
        let v_buf = cache.v.take().unwrap().slice_assign(
            [0..batch, 0..self.n_kv_heads, pos..pos_end, 0..self.d_head],
            v,
        );
        cache.len = pos_end;

        // Slice active portion (zero-copy view), then put buffer back
        let k = k_buf.clone().slice([0..batch, 0..self.n_kv_heads, 0..pos_end, 0..self.d_head]);
        let v = v_buf.clone().slice([0..batch, 0..self.n_kv_heads, 0..pos_end, 0..self.d_head]);
        cache.k = Some(k_buf);
        cache.v = Some(v_buf);

        // GQA: repeat KV heads to match Q heads
        let (k, v) = if self.n_q_heads != self.n_kv_heads {
            let n_rep = self.n_q_heads / self.n_kv_heads;
            (repeat_kv(k, n_rep), repeat_kv(v, n_rep))
        } else {
            (k, v)
        };

        // Scaled dot-product attention via burn's fused attention kernel.
        // scale=None uses default 1/sqrt(d_head), which avoids the fallback path.
        // is_causal=true during prefill (seq_len > 1), false during single-token decode.
        let options = AttentionModuleOptions {
            is_causal,
            ..Default::default()
        };
        let out = attention(q, k, v, None, None, options);

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
