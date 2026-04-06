use burn::tensor::{Int, Tensor, TensorData};

use crate::B;

const BLOCK_SIZE: usize = 128;
const BLOCK_BYTES: usize = 18;

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
