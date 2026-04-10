use burn::module::Param;
use burn::prelude::*;

/// Simulate Q1_0_g128 quantization then dequantize back to f32.
///
/// For each group of 128 elements along dim 1:
///   scale = mean(abs(group))
///   quantized[i] = if weight[i] >= 0 { +scale } else { -scale }
///
/// This is the "fake quantization" used during STE training: the forward
/// pass sees quantized values, but gradients flow through as identity.
pub fn quantize_dequantize_q1_g128<B: Backend>(w: Tensor<B, 2>) -> Tensor<B, 2> {
    let [d_out, d_in] = w.dims();
    let n_groups = d_in / 128;

    // Reshape to [d_out * n_groups, 128] so each row is one quantization group
    let flat = w.clone().reshape([d_out * n_groups, 128]);

    // Scale per group = mean(abs(elements))
    let scales = flat.clone().abs().mean_dim(1); // [d_out * n_groups, 1]

    // Sign: +1.0 for >= 0, -1.0 for < 0
    let signs = flat.clone().sign();

    // Reconstructed: each element is ±scale
    let reconstructed = signs * scales;

    reconstructed.reshape([d_out, d_in])
}

/// Linear layer with Straight-Through Estimation for 1-bit weights.
///
/// Stores FP32 shadow weights (the optimization target). During forward:
/// - Computes `w + (quantize(w) - w).detach()` — forward value equals
///   the quantized weights, but gradient w.r.t. shadow weights is identity.
/// - Then performs standard matmul with the STE weights.
///
/// This ensures the model trains on exactly what it will use at inference
/// (quantized ±scale values) while maintaining a smooth optimization surface.
#[derive(Module, Debug)]
pub struct SteLinear<B: Backend> {
    pub weight: Param<Tensor<B, 2>>, // [d_out, d_in]
}

impl<B: Backend> SteLinear<B> {
    /// Create from an existing weight tensor.
    pub fn from_weight(weight: Tensor<B, 2>) -> Self {
        SteLinear {
            weight: Param::from_tensor(weight),
        }
    }

    /// Forward pass with STE quantization.
    ///
    /// The trick: `w + (quantize(w) - w).detach()`
    /// - Forward value = quantize(w) (since w + quantize(w) - w = quantize(w))
    /// - Gradient w.r.t. w = 1 (since detach makes (quantize(w) - w) a constant)
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let w = self.weight.val();
        let [d_out, d_in] = w.dims();

        let quantized = quantize_dequantize_q1_g128(w.clone());
        let ste_w = w.clone() + (quantized - w).detach();

        // Flatten input to 2D for matmul, then reshape back
        let in_dims = input.dims();
        let batch: usize = in_dims[..D - 1].iter().product();
        let input_2d: Tensor<B, 2> = input.reshape([batch, d_in]);
        let output_2d = input_2d.matmul(ste_w.transpose());
        output_2d.reshape({
            let mut out_dims = in_dims;
            out_dims[D - 1] = d_out;
            out_dims
        })
    }
}
