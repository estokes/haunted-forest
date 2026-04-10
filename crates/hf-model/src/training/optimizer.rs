use crate::bit_linear::BitLinear;
use crate::B;
use burn::tensor::Tensor;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const BLOCK_SIZE: usize = 128;

/// Adam optimizer state for one weight matrix, stored on CPU.
pub struct AdamState {
    pub m: Vec<f32>,  // first moment
    pub v: Vec<f32>,  // second moment
}

impl AdamState {
    pub fn zeros(n: usize) -> Self {
        AdamState {
            m: vec![0.0; n],
            v: vec![0.0; n],
        }
    }
}

/// Shadow weights + Adam state for all projections in one layer.
pub struct LayerOptState {
    pub q_proj: (Vec<f32>, AdamState),
    pub k_proj: (Vec<f32>, AdamState),
    pub v_proj: (Vec<f32>, AdamState),
    pub o_proj: (Vec<f32>, AdamState),
    pub ffn_gate: (Vec<f32>, AdamState),
    pub ffn_up: (Vec<f32>, AdamState),
    pub ffn_down: (Vec<f32>, AdamState),
}

/// Adam hyperparameters.
pub struct AdamConfig {
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub eps: f32,
}

impl Default for AdamConfig {
    fn default() -> Self {
        AdamConfig {
            lr: 1e-4,
            beta1: 0.9,
            beta2: 0.999,
            eps: 1e-8,
        }
    }
}

/// Initialize shadow weights for one layer by dequantizing its packed BitLinear weights.
pub fn init_layer_opt_state(
    q_proj: &BitLinear,
    k_proj: &BitLinear,
    v_proj: &BitLinear,
    o_proj: &BitLinear,
    ffn_gate: &BitLinear,
    ffn_up: &BitLinear,
    ffn_down: &BitLinear,
) -> LayerOptState {
    fn dequant_to_cpu(bl: &BitLinear) -> (Vec<f32>, AdamState) {
        let w = bl.dequantize_weights();
        let data: Vec<f32> = w.into_data().to_vec().unwrap();
        let n = data.len();
        (data, AdamState::zeros(n))
    }

    LayerOptState {
        q_proj: dequant_to_cpu(q_proj),
        k_proj: dequant_to_cpu(k_proj),
        v_proj: dequant_to_cpu(v_proj),
        o_proj: dequant_to_cpu(o_proj),
        ffn_gate: dequant_to_cpu(ffn_gate),
        ffn_up: dequant_to_cpu(ffn_up),
        ffn_down: dequant_to_cpu(ffn_down),
    }
}

/// Fused Adam step + re-quantize for one projection.
///
/// In a single pass over memory: update shadow/m/v via Adam, then quantize
/// the updated shadow weights to packed Q1_0_g128. This halves memory traffic
/// compared to separate Adam + quantize passes.
///
/// Uses AVX-512 intrinsics for the Adam inner loop on Zen 4+.
pub fn adam_step_projection(
    shadow: &mut Vec<f32>,
    adam: &mut AdamState,
    grad_gpu: Tensor<B, 2>,
    bl: &mut BitLinear,
    cfg: &AdamConfig,
    t: usize,
    device: &burn::backend::cuda::CudaDevice,
) {
    let grad: Vec<f32> = grad_gpu.into_data().to_vec().unwrap();
    assert_eq!(shadow.len(), grad.len());

    let n = shadow.len();
    let n_blocks = (n + BLOCK_SIZE - 1) / BLOCK_SIZE;
    let beta1_t = 1.0 - cfg.beta1.powi(t as i32);
    let beta2_t = 1.0 - cfg.beta2.powi(t as i32);

    let mut packed_words = vec![0i32; n_blocks * 4];
    let mut scales = vec![0.0f32; n_blocks];

    let n_threads = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);
    // Chunk by BLOCK_SIZE boundaries so each thread handles complete quant blocks
    let blocks_per_thread = (n_blocks + n_threads - 1) / n_threads;

    std::thread::scope(|s| {
        let pw_chunks = packed_words.chunks_mut(blocks_per_thread * 4);
        let sc_chunks = scales.chunks_mut(blocks_per_thread);
        let sh_chunks = shadow.chunks_mut(blocks_per_thread * BLOCK_SIZE);
        let m_chunks = adam.m.chunks_mut(blocks_per_thread * BLOCK_SIZE);
        let v_chunks = adam.v.chunks_mut(blocks_per_thread * BLOCK_SIZE);
        let g_chunks = grad.chunks(blocks_per_thread * BLOCK_SIZE);

        for ((((pw, sc), sh), (mi, vi)), gi) in
            pw_chunks.zip(sc_chunks)
            .zip(sh_chunks)
            .zip(m_chunks.zip(v_chunks))
            .zip(g_chunks)
        {
            let beta1 = cfg.beta1;
            let beta2 = cfg.beta2;
            let lr = cfg.lr;
            let eps = cfg.eps;
            s.spawn(move || {
                fused_adam_quantize_chunk(
                    sh, mi, vi, gi, pw, sc,
                    beta1, beta2, lr, eps, beta1_t, beta2_t,
                );
            });
        }
    });

    bl.update_packed(packed_words, scales, device);
}

/// Fused Adam + quantize for one thread's chunk of blocks.
///
/// Processes BLOCK_SIZE (128) elements at a time: first runs Adam to update
/// shadow/m/v, then quantizes the 128 updated shadow values into one packed
/// block (scale + 4 × u32 words).
fn fused_adam_quantize_chunk(
    shadow: &mut [f32],
    m: &mut [f32],
    v: &mut [f32],
    grad: &[f32],
    packed_words: &mut [i32],
    scales: &mut [f32],
    beta1: f32, beta2: f32, lr: f32, eps: f32,
    beta1_t: f32, beta2_t: f32,
) {
    let n_blocks = scales.len();

    for b in 0..n_blocks {
        let start = b * BLOCK_SIZE;
        let end = (start + BLOCK_SIZE).min(shadow.len());
        let len = end - start;

        // Phase 1: Adam update for this block's 128 elements
        adam_block_avx512(
            &mut shadow[start..end],
            &mut m[start..end],
            &mut v[start..end],
            &grad[start..end],
            beta1, beta2, lr, eps, beta1_t, beta2_t,
        );

        // Phase 2: Quantize the updated shadow values (still in cache from phase 1)
        let group = &shadow[start..end];
        let scale: f32 = group.iter().map(|x| x.abs()).sum::<f32>() / len as f32;
        scales[b] = scale;

        let mut w32 = [0u32; 4];
        for (j, &val) in group.iter().enumerate() {
            if val >= 0.0 {
                w32[j / 32] |= 1 << (j % 32);
            }
        }
        for (k, word) in w32.iter().enumerate() {
            packed_words[b * 4 + k] = *word as i32;
        }
    }
}

/// AVX-512 Adam update for one block of elements.
///
/// Processes 16 f32s per iteration (512 bits). Falls back to scalar for
/// the tail if the block isn't a multiple of 16.
#[cfg(target_arch = "x86_64")]
fn adam_block_avx512(
    shadow: &mut [f32],
    m: &mut [f32],
    v: &mut [f32],
    grad: &[f32],
    beta1: f32, beta2: f32, lr: f32, eps: f32,
    beta1_t: f32, beta2_t: f32,
) {
    let n = shadow.len();
    let simd_len = n / 16 * 16;

    if simd_len > 0 {
        unsafe {
            let b1 = _mm512_set1_ps(beta1);
            let b2 = _mm512_set1_ps(beta2);
            let one_minus_b1 = _mm512_set1_ps(1.0 - beta1);
            let one_minus_b2 = _mm512_set1_ps(1.0 - beta2);
            let lr_v = _mm512_set1_ps(lr);
            let eps_v = _mm512_set1_ps(eps);
            let b1t = _mm512_set1_ps(beta1_t);
            let b2t = _mm512_set1_ps(beta2_t);

            let mut i = 0;
            while i < simd_len {
                let g = _mm512_loadu_ps(grad.as_ptr().add(i));

                // m = beta1 * m + (1 - beta1) * g
                let mi = _mm512_loadu_ps(m.as_ptr().add(i));
                let mi = _mm512_fmadd_ps(b1, mi, _mm512_mul_ps(one_minus_b1, g));
                _mm512_storeu_ps(m.as_mut_ptr().add(i), mi);

                // v = beta2 * v + (1 - beta2) * g * g
                let vi = _mm512_loadu_ps(v.as_ptr().add(i));
                let vi = _mm512_fmadd_ps(b2, vi, _mm512_mul_ps(one_minus_b2, _mm512_mul_ps(g, g)));
                _mm512_storeu_ps(v.as_mut_ptr().add(i), vi);

                // m_hat = m / beta1_t, v_hat = v / beta2_t
                let m_hat = _mm512_div_ps(mi, b1t);
                let v_hat = _mm512_div_ps(vi, b2t);

                // shadow -= lr * m_hat / (sqrt(v_hat) + eps)
                let denom = _mm512_add_ps(_mm512_sqrt_ps(v_hat), eps_v);
                let update = _mm512_mul_ps(lr_v, _mm512_div_ps(m_hat, denom));
                let si = _mm512_loadu_ps(shadow.as_ptr().add(i));
                let si = _mm512_sub_ps(si, update);
                _mm512_storeu_ps(shadow.as_mut_ptr().add(i), si);

                i += 16;
            }
        }
    }

    // Scalar tail
    for i in simd_len..n {
        let g = grad[i];
        m[i] = beta1 * m[i] + (1.0 - beta1) * g;
        v[i] = beta2 * v[i] + (1.0 - beta2) * g * g;
        let m_hat = m[i] / beta1_t;
        let v_hat = v[i] / beta2_t;
        shadow[i] -= lr * m_hat / (v_hat.sqrt() + eps);
    }
}
