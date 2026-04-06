use burn::tensor::Tensor;
use crate::B;

/// Kernel module — isolates cubecl prelude from burn tensor namespace.
mod kernel {
    use burn::cubecl::prelude::*;
    use burn_cubecl::{
        CubeRuntime,
        tensor::CubeTensor,
        ops::numeric::empty_device_dtype,
    };
    use burn_backend::{DType, Shape};

    #[cube(launch_unchecked)]
    fn rms_norm_kernel<F: Float>(
        input: &Tensor<F>,
        gamma: &Tensor<F>,
        output: &mut Tensor<F>,
        #[comptime] d: u32,
        #[comptime] eps_x1e10: u32,
    ) {
        let row = CUBE_POS_X as usize;
        let tid = UNIT_POS as usize;
        let d_us = d as usize;
        let elems_per_thread = d_us / 32;
        let row_base = row * d_us;

        let eps = F::cast_from(eps_x1e10) / F::new(1e10);

        // Phase 1: sum of squares
        let mut sum_sq = F::new(0.0);
        for i in 0..elems_per_thread {
            let val = input[row_base + tid + i * 32];
            sum_sq += val * val;
        }
        let total_sq = plane_sum(sum_sq);
        let inv_rms = F::new(1.0) / F::sqrt(total_sq / F::cast_from(d) + eps);

        // Phase 2: normalize and scale by gamma
        for i in 0..elems_per_thread {
            let idx = row_base + tid + i * 32;
            let g_idx = tid + i * 32;
            output[idx] = input[idx] * gamma[g_idx] * inv_rms;
        }
    }

    pub fn launch_rms_norm<R: CubeRuntime>(
        input: CubeTensor<R>,
        gamma: CubeTensor<R>,
        d: usize,
        n_rows: usize,
        eps: f32,
    ) -> CubeTensor<R> {
        assert!(d % 32 == 0, "d ({d}) must be a multiple of 32");

        let client = input.client.clone();
        let output = empty_device_dtype(
            client.clone(),
            input.device.clone(),
            Shape::from([n_rows * d]),
            DType::F32,
        );

        let cube_dim = CubeDim::new_1d(32);
        let cube_count = CubeCount::new_1d(n_rows as u32);

        let eps_x1e10 = (eps as f64 * 1e10).round() as u32;

        unsafe {
            rms_norm_kernel::launch_unchecked::<f32, R>(
                &client,
                cube_count,
                cube_dim,
                input.into_tensor_arg(),
                gamma.into_tensor_arg(),
                output.clone().into_tensor_arg(),
                d as u32,
                eps_x1e10,
            );
        }

        output
    }
}

/// Fused RmsNorm layer using a single cubecl kernel per call.
#[derive(Debug)]
pub struct FusedRmsNorm {
    pub gamma: Tensor<B, 1>,
    pub d: usize,
    pub epsilon: f64,
}

impl FusedRmsNorm {
    /// Create with ones (placeholder, will be replaced by loader).
    pub fn empty(d: usize, epsilon: f64, device: &burn::backend::cuda::CudaDevice) -> Self {
        FusedRmsNorm {
            gamma: Tensor::ones([d], device),
            d,
            epsilon,
        }
    }

    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let dims = input.dims();
        let d = dims[D - 1];
        assert_eq!(d, self.d);
        let n_rows: usize = dims[..D - 1].iter().product();

        let input_flat: Tensor<B, 1> = input.reshape([n_rows * d]);
        let input_cube = input_flat.into_primitive().tensor();
        let gamma_cube = self.gamma.clone().into_primitive().tensor();

        let output_cube = kernel::launch_rms_norm(
            input_cube,
            gamma_cube,
            d,
            n_rows,
            self.epsilon as f32,
        );

        let output_flat = Tensor::<B, 1>::from_primitive(
            burn::tensor::TensorPrimitive::Float(output_cube),
        );
        output_flat.reshape(dims)
    }
}
