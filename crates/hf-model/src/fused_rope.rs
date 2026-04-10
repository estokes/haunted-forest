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

    /// Fused RoPE kernel: applies 2x2 rotation to adjacent element pairs.
    ///
    /// Input is flattened [n_rows, d_head] where each row is one head at one
    /// position. The kernel computes the sequence position from the row index
    /// and reads the precomputed cos/sin tables.
    ///
    /// Grid: one warp (32 threads) per row.
    /// Each thread handles (d_head/2)/32 rotation pairs.
    #[cube(launch_unchecked)]
    fn rope_kernel<F: Float>(
        input: &Tensor<F>,
        cos_table: &Tensor<F>,
        sin_table: &Tensor<F>,
        output: &mut Tensor<F>,
        #[comptime] half_d: u32,
        n_heads: u32,
        seq_len: u32,
        offset: u32,
    ) {
        let row = CUBE_POS_X as usize;
        let tid = UNIT_POS as usize;
        let half_d_us = half_d as usize;
        let d_head = half_d_us * 2;
        let pairs_per_thread = half_d_us / 32;

        // Determine sequence position for this row.
        // Input layout: [batch, seq_len, n_heads, d_head] flattened to [n_rows, d_head]
        // row r → seq_idx = (r / n_heads) % seq_len
        let seq_idx = ((row as u32 / n_heads) % seq_len) as usize;
        let pos_idx = offset as usize + seq_idx;

        let row_base = row * d_head;
        let trig_base = pos_idx * half_d_us;

        for i in 0..pairs_per_thread {
            let pair_idx = tid + i * 32;
            let cos_val = cos_table[trig_base + pair_idx];
            let sin_val = sin_table[trig_base + pair_idx];

            let even_idx = row_base + pair_idx * 2;
            let odd_idx = even_idx + 1;

            let x_even = input[even_idx];
            let x_odd = input[odd_idx];

            output[even_idx] = x_even * cos_val - x_odd * sin_val;
            output[odd_idx] = x_even * sin_val + x_odd * cos_val;
        }
    }

    pub fn launch_rope<R: CubeRuntime>(
        input: CubeTensor<R>,
        cos_table: CubeTensor<R>,
        sin_table: CubeTensor<R>,
        d_head: usize,
        n_heads: usize,
        seq_len: usize,
        offset: usize,
        n_rows: usize,
    ) -> CubeTensor<R> {
        let half_d = d_head / 2;
        assert!(half_d % 32 == 0, "d_head/2 ({half_d}) must be a multiple of 32");

        let client = input.client.clone();
        let output = empty_device_dtype(
            client.clone(),
            input.device.clone(),
            Shape::from([n_rows * d_head]),
            DType::F32,
        );

        let cube_dim = CubeDim::new_1d(32);
        let cube_count = CubeCount::new_1d(n_rows as u32);

        unsafe {
            rope_kernel::launch_unchecked::<f32, R>(
                &client,
                cube_count,
                cube_dim,
                input.into_tensor_arg(),
                cos_table.into_tensor_arg(),
                sin_table.into_tensor_arg(),
                output.clone().into_tensor_arg(),
                half_d as u32,
                n_heads as u32,
                seq_len as u32,
                offset as u32,
            );
        }

        output
    }
}

/// Fused RoPE layer using a single cubecl kernel per apply() call.
///
/// Replaces burn's RotaryEncoding::apply() which generates 8-10 kernel
/// launches per call. Precomputes cos/sin tables from the RotaryEncoding's
/// frequency tensor (which already includes any YARN scaling).
#[derive(Debug)]
pub struct FusedRoPE {
    pub cos: Tensor<B, 1>,      // [max_seq_len * d_head/2] flattened
    pub sin: Tensor<B, 1>,      // [max_seq_len * d_head/2] flattened
    pub neg_sin: Tensor<B, 1>,  // negated sin for backward (inverse rotation)
    pub d_head: usize,
}

impl FusedRoPE {
    /// Create from a RotaryEncoding by recomputing cos/sin from its theta.
    ///
    /// Uses rope.theta (which already includes frequency scaling) to build
    /// flat cos/sin lookup tables of shape [max_seq_len * d_head/2].
    pub fn from_rope(
        rope: &burn::nn::RotaryEncoding<B>,
        d_head: usize,
        max_seq_len: usize,
        device: &burn::backend::cuda::CudaDevice,
    ) -> Self {
        use burn::tensor::Int;

        let half_d = d_head / 2;

        // positions: [max_seq_len, 1]
        let positions = Tensor::<B, 1, Int>::arange(0..max_seq_len as i64, device)
            .float()
            .reshape([max_seq_len, 1]);

        // theta: [1, d_head/2] — already includes YARN frequency scaling
        let theta: Tensor<B, 2> = rope.theta.clone().reshape([1, half_d]);

        // frequencies[pos, k] = pos * theta[k]
        let frequencies = positions * theta;

        let cos: Tensor<B, 1> = frequencies.clone().cos().reshape([max_seq_len * half_d]);
        let sin: Tensor<B, 1> = frequencies.sin().reshape([max_seq_len * half_d]);
        let neg_sin = sin.clone().neg();

        FusedRoPE { cos, sin, neg_sin, d_head }
    }

    /// Apply rotary positional encoding in a single kernel launch.
    ///
    /// Input shape: `[batch, seq_len, n_heads, d_head]` (BEFORE swap_dims).
    /// The d_head dimension must be contiguous (true for reshape output).
    pub fn apply(&self, input: Tensor<B, 4>, pos: usize) -> Tensor<B, 4> {
        let dims = input.dims();
        let [batch, seq_len, n_heads, d_head] = dims;
        assert_eq!(d_head, self.d_head);

        let n_rows = batch * seq_len * n_heads;
        let input_flat: Tensor<B, 1> = input.reshape([n_rows * d_head]);

        let input_cube = input_flat.into_primitive().tensor();
        let cos_cube = self.cos.clone().into_primitive().tensor();
        let sin_cube = self.sin.clone().into_primitive().tensor();

        let output_cube = kernel::launch_rope(
            input_cube,
            cos_cube,
            sin_cube,
            d_head,
            n_heads,
            seq_len,
            pos,
            n_rows,
        );

        let output_flat = Tensor::<B, 1>::from_primitive(
            burn::tensor::TensorPrimitive::Float(output_cube),
        );
        output_flat.reshape(dims)
    }

    /// Apply inverse rotary encoding (for backward pass).
    ///
    /// Uses negated sin table: rotation by -θ instead of +θ.
    /// Same input layout as `apply`: `[batch, seq_len, n_heads, d_head]`.
    pub fn apply_inverse(&self, input: Tensor<B, 4>, pos: usize) -> Tensor<B, 4> {
        let dims = input.dims();
        let [batch, seq_len, n_heads, d_head] = dims;
        assert_eq!(d_head, self.d_head);

        let n_rows = batch * seq_len * n_heads;
        let input_flat: Tensor<B, 1> = input.reshape([n_rows * d_head]);

        let input_cube = input_flat.into_primitive().tensor();
        let cos_cube = self.cos.clone().into_primitive().tensor();
        let neg_sin_cube = self.neg_sin.clone().into_primitive().tensor();

        let output_cube = kernel::launch_rope(
            input_cube,
            cos_cube,
            neg_sin_cube, // negated sin → inverse rotation
            d_head,
            n_heads,
            seq_len,
            pos,
            n_rows,
        );

        let output_flat = Tensor::<B, 1>::from_primitive(
            burn::tensor::TensorPrimitive::Float(output_cube),
        );
        output_flat.reshape(dims)
    }
}
