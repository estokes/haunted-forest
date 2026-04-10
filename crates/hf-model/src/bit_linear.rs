use burn::tensor::{Int, Tensor, TensorData};

use crate::B;

const BLOCK_SIZE: usize = 128;
const BLOCK_BYTES: usize = 18;

/// GPU-resident dequantization kernel for Q1_0_g128 packed weights.
mod dequant_kernel {
    use burn::cubecl::prelude::*;
    use burn_cubecl::{
        CubeRuntime,
        tensor::CubeTensor,
        ops::numeric::empty_device_dtype,
    };
    use burn_backend::{DType, Shape};

    /// Each thread dequantizes one element from packed Q1_0_g128 format.
    /// bit=1 → +scale, bit=0 → -scale.
    #[cube(launch_unchecked)]
    fn dequant<F: Float>(
        packed: &Tensor<u32>,
        scales: &Tensor<F>,
        output: &mut Tensor<F>,
        n_elements: u32,
    ) {
        let idx = (CUBE_POS_X * CUBE_DIM_X + UNIT_POS_X) as usize;
        if idx >= n_elements as usize {
            terminate!();
        }

        let block_idx = idx / 128;
        let elem_in_block = idx % 128;
        let word_idx = elem_in_block / 32;
        let bit_idx = (elem_in_block % 32) as u32;

        let word = packed[block_idx * 4 + word_idx];
        let bit = (word >> bit_idx) & 1u32;
        let scale = scales[block_idx];

        let two = F::new(2.0);
        let one = F::new(1.0);
        output[idx] = (F::cast_from(bit) * two - one) * scale;
    }

    pub fn launch_dequant<R: CubeRuntime>(
        packed_words: CubeTensor<R>,
        scales: CubeTensor<R>,
        n_elements: usize,
    ) -> CubeTensor<R> {
        let client = scales.client.clone();
        let output = empty_device_dtype(
            client.clone(),
            scales.device.clone(),
            Shape::from([n_elements]),
            DType::F32,
        );

        // Reinterpret i32 packed words as u32
        let packed_u32 = CubeTensor::new(
            packed_words.client,
            packed_words.handle,
            *packed_words.meta,
            packed_words.device,
            DType::U32,
        );

        let threads = 256u32;
        let blocks = ((n_elements as u32) + threads - 1) / threads;
        let cube_dim = CubeDim::new_1d(threads);
        let cube_count = CubeCount::new_1d(blocks);

        unsafe {
            dequant::launch_unchecked::<f32, R>(
                &client,
                cube_count,
                cube_dim,
                packed_u32.into_tensor_arg(),
                scales.into_tensor_arg(),
                output.clone().into_tensor_arg(),
                n_elements as u32,
            );
        }

        output
    }
}

/// A linear layer that stores weights in packed Q1_0_g128 format on the GPU
/// and dequantizes to f32 on-the-fly during each forward pass.
///
/// The packed data (~0.14 bytes/element) is GPU-resident, avoiding any
/// per-forward PCIe transfer. During forward(), bitwise tensor ops extract
/// bits and multiply by scales to produce a transient f32 weight matrix
/// that is freed after matmul.
#[derive(Debug)]
pub struct BitLinear {
    packed_words: Tensor<B, 2, Int>, // [n_blocks, 4]
    scales: Tensor<B, 1>,           // [n_blocks] f32
    n_elements: usize,
    /// Shape in gguf convention: [ne0, ne1].
    /// ne0 = d_input (stride-1 in ggml column-major).
    /// Transpose to [d_input, d_output] happens in forward().
    shape: [usize; 2],
}

impl BitLinear {
    /// Placeholder — will be replaced by loader with actual packed data.
    pub fn empty(device: &burn::backend::cuda::CudaDevice) -> Self {
        BitLinear {
            packed_words: Tensor::zeros([1, 4], device),
            scales: Tensor::zeros([1], device),
            n_elements: 0,
            shape: [0, 0],
        }
    }

    /// Build from raw Q1_0_g128 packed bytes, uploading directly to GPU.
    ///
    /// Separates the interleaved scale+bits layout and stores both as
    /// GPU-resident tensors. The raw bytes can be dropped after this call.
    pub fn from_packed(packed: &[u8], n_elements: usize, shape: [usize; 2], device: &burn::backend::cuda::CudaDevice) -> Self {
        let n_blocks = (n_elements + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let mut words = Vec::with_capacity(n_blocks * 4);
        let mut scales_vec = Vec::with_capacity(n_blocks);

        for i in 0..n_blocks {
            let block = &packed[i * BLOCK_BYTES..];
            scales_vec.push(half::f16::from_le_bytes([block[0], block[1]]).to_f32());
            for w in 0..4 {
                let off = 2 + w * 4;
                words.push(i32::from_le_bytes([
                    block[off], block[off + 1], block[off + 2], block[off + 3],
                ]));
            }
        }

        BitLinear {
            packed_words: Tensor::from_data(TensorData::new(words, [n_blocks, 4]), device),
            scales: Tensor::from_data(TensorData::new(scales_vec, [n_blocks]), device),
            n_elements,
            shape,
        }
    }

    /// Dequantize packed Q1_0_g128 weights to a full FP32 tensor [d_out, d_in].
    ///
    /// Used for converting to training model. This is expensive — reads all
    /// packed data from GPU, dequantizes on CPU, then uploads the f32 result.
    pub fn dequantize_weights(&self) -> Tensor<B, 2> {
        let ne0 = self.shape[0]; // d_in
        let ne1 = self.shape[1]; // d_out
        let n_elements = self.n_elements;
        let n_blocks = (n_elements + BLOCK_SIZE - 1) / BLOCK_SIZE;

        // Read packed data from GPU
        let words_data = self.packed_words.clone().into_data();
        let scales_data = self.scales.clone().into_data();

        let words: Vec<i32> = words_data.to_vec().unwrap();
        let scales: Vec<f32> = scales_data.to_vec().unwrap();

        // Dequantize on CPU
        let mut out = Vec::with_capacity(n_elements);
        for b in 0..n_blocks {
            let scale = scales[b];
            let w = [
                words[b * 4] as u32,
                words[b * 4 + 1] as u32,
                words[b * 4 + 2] as u32,
                words[b * 4 + 3] as u32,
            ];
            let remaining = n_elements - out.len();
            let count = remaining.min(BLOCK_SIZE);
            for j in 0..count {
                let word_idx = j / 32;
                let bit_idx = j % 32;
                let bit = (w[word_idx] >> bit_idx) & 1;
                out.push(if bit != 0 { scale } else { -scale });
            }
        }

        // Upload as [ne0, ne1] (gguf convention), then transpose to [ne1, ne0] = [d_out, d_in]
        let device = self.scales.device();
        let t: Tensor<B, 2> = Tensor::from_data(TensorData::new(out, [ne1, ne0]), &device);
        t
    }

    /// Quantize FP32 weights to packed Q1_0_g128 and upload to GPU.
    ///
    /// For each group of 128 elements: scale = mean(abs(group)),
    /// bit = 1 if weight >= 0, 0 otherwise.
    pub fn from_f32_weights(w: Tensor<B, 2>, device: &burn::backend::cuda::CudaDevice) -> Self {
        let [d_out, d_in] = w.dims();
        let n_elements = d_out * d_in;
        let n_blocks = (n_elements + BLOCK_SIZE - 1) / BLOCK_SIZE;

        // Read weights to CPU
        let w_data: Vec<f32> = w.into_data().to_vec().unwrap();

        let mut words = Vec::with_capacity(n_blocks * 4);
        let mut scales_vec = Vec::with_capacity(n_blocks);

        for b in 0..n_blocks {
            let start = b * BLOCK_SIZE;
            let end = (start + BLOCK_SIZE).min(n_elements);
            let group = &w_data[start..end];

            // Scale = mean(abs(group))
            let scale: f32 = group.iter().map(|v| v.abs()).sum::<f32>() / group.len() as f32;
            scales_vec.push(scale);

            // Pack bits into 4 u32 words
            let mut w32 = [0u32; 4];
            for (j, &val) in group.iter().enumerate() {
                if val >= 0.0 {
                    w32[j / 32] |= 1 << (j % 32);
                }
            }
            for word in w32 {
                words.push(word as i32);
            }
        }

        BitLinear {
            packed_words: Tensor::from_data(TensorData::new(words, [n_blocks, 4]), device),
            scales: Tensor::from_data(TensorData::new(scales_vec, [n_blocks]), device),
            n_elements,
            shape: [d_in, d_out],
        }
    }

    /// Weight shape in [d_in, d_out] (GGUF convention: ne0=d_in, ne1=d_out).
    pub fn shape(&self) -> [usize; 2] { self.shape }

    /// Dequantize packed weights to f32 entirely on GPU (no CPU roundtrip).
    ///
    /// Returns a [d_out, d_in] f32 tensor. The temporary uses ~23 MB for the
    /// largest projection (5632×2048). Used during backward pass.
    pub fn dequantize_weights_gpu(&self) -> Tensor<B, 2> {
        let ne0 = self.shape[0]; // d_in
        let ne1 = self.shape[1]; // d_out
        let n_blocks = self.scales.dims()[0];

        let packed_flat: Tensor<B, 1, Int> = self.packed_words.clone().reshape([n_blocks * 4]);
        let scales_flat: Tensor<B, 1> = self.scales.clone();

        let packed_cube = packed_flat.into_primitive();
        let scales_cube = scales_flat.into_primitive().tensor();

        let output_cube = dequant_kernel::launch_dequant(
            packed_cube,
            scales_cube,
            self.n_elements,
        );

        let output_flat = Tensor::<B, 1>::from_primitive(
            burn::tensor::TensorPrimitive::Float(output_cube),
        );
        // Packed data is in [ne1, ne0] = [d_out, d_in] row-major order
        output_flat.reshape([ne1, ne0])
    }

    /// Number of output features (d_out).
    pub fn d_out(&self) -> usize { self.shape[1] }

    /// Number of input features (d_in).
    pub fn d_in(&self) -> usize { self.shape[0] }

    /// Replace packed GPU data from CPU-computed quantization.
    pub fn update_packed(&mut self, packed_words: Vec<i32>, scales: Vec<f32>, device: &burn::backend::cuda::CudaDevice) {
        let n_blocks = scales.len();
        self.packed_words = Tensor::from_data(TensorData::new(packed_words, [n_blocks, 4]), device);
        self.scales = Tensor::from_data(TensorData::new(scales, [n_blocks]), device);
    }

    /// Forward pass: fused 1-bit dequant + GEMV via cubecl kernel.
    ///
    /// Reads packed weights directly during the dot product — no intermediate
    /// f32 weight tensor is materialized. Only ~0.14 bytes/element of packed
    /// data is read from VRAM instead of 4 bytes/element.
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let ne0 = self.shape[0]; // d_in
        let ne1 = self.shape[1]; // d_out
        let dims = input.dims();
        let n_rows: usize = dims[..D - 1].iter().product();

        let n_blocks = self.scales.dims()[0];

        // Flatten everything to 1D in burn-land before extracting primitives
        let input_flat: Tensor<B, 1> = input.reshape([n_rows * ne0]);
        let packed_flat: Tensor<B, 1, Int> = self.packed_words.clone().reshape([n_blocks * 4]);
        let scales_flat: Tensor<B, 1> = self.scales.clone();

        // Extract CubeTensors
        let input_cube = input_flat.into_primitive().tensor();
        let packed_cube = packed_flat.into_primitive();
        let scales_cube = scales_flat.into_primitive().tensor();

        // Launch fused kernel
        let output_cube = crate::fused_gemv::launch(
            packed_cube, scales_cube, input_cube, ne0, ne1, n_rows,
        );

        // Wrap back into burn Tensor and reshape to original dims with d_out
        let output_flat = Tensor::<B, 1>::from_primitive(
            burn::tensor::TensorPrimitive::Float(output_cube),
        );

        let mut out_shape: [usize; D] = dims;
        out_shape[D - 1] = ne1;
        output_flat.reshape(out_shape)
    }
}
