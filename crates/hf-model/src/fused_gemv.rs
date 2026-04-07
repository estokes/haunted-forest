use burn::cubecl::prelude::*;
use burn_cubecl::{
    CubeRuntime,
    tensor::CubeTensor,
    ops::numeric::empty_device_dtype,
};
use burn_backend::{DType, Shape, TensorMetadata};

/// Fused 1-bit dequant + GEMV kernel.
///
/// Grid: one warp (32 threads) per output element.
/// - CUBE_POS_X = output column j (0..d_out)
/// - CUBE_POS_Y = input row (0..n_rows, batch*seq flattened)
/// - UNIT_POS = lane within warp (0..31)
///
/// Each warp computes one dot product by iterating over 128-element blocks.
/// All 32 threads load the same 4 packed u32 words (L1 broadcast), each
/// thread extracts its bit, multiplies by scale and input, then plane_sum
/// reduces across the warp.
/// Fused 1-bit dequant + GEMV kernel (multi-warp).
///
/// Each block has WARPS_PER_BLOCK warps (128 threads for 4 warps).
/// Each warp computes one output element independently.
/// All warps in a block read the same input row → shared L1 cache.
///
/// CUBE_POS_X = block index for output columns (each block handles WARPS_PER_BLOCK columns)
/// CUBE_POS_Y = input row
/// UNIT_POS_X = lane within warp (0..31)
/// UNIT_POS_Y = warp index within block (0..WARPS_PER_BLOCK-1)
#[cube(launch_unchecked)]
fn q1_fused_gemv<F: Float>(
    packed: &Tensor<u32>,
    scales: &Tensor<F>,
    input: &Tensor<F>,
    output: &mut Tensor<F>,
    #[comptime] blocks_per_row: u32,
    #[comptime] warps_per_block: u32,
    d_in: u32,
    d_out: u32,
) {
    let warp_id = UNIT_POS_Y;
    let j = (CUBE_POS_X * warps_per_block + warp_id) as usize;
    let row = CUBE_POS_Y as usize;
    let lane = UNIT_POS_X as usize;

    let d_out_us = d_out as usize;

    // Guard: last block may have fewer than warps_per_block valid outputs
    if j >= d_out_us {
        terminate!();
    }

    let blocks_per_row_us = blocks_per_row as usize;
    let d_in_us = d_in as usize;

    let mut sum = F::new(0.0);

    let packed_base = j * blocks_per_row_us * 4;
    let scale_base = j * blocks_per_row_us;
    let input_base = row * d_in_us;

    let two = F::new(2.0);
    let one = F::new(1.0);

    for block in 0..blocks_per_row {
        let b = block as usize;
        let scale = scales[scale_base + b];
        let wb = packed_base + b * 4;
        let w0 = packed[wb];
        let w1 = packed[wb + 1];
        let w2 = packed[wb + 2];
        let w3 = packed[wb + 3];

        let lane32 = UNIT_POS_X;
        let s0 = F::cast_from((w0 >> lane32) & 1u32) * two - one;
        let s1 = F::cast_from((w1 >> lane32) & 1u32) * two - one;
        let s2 = F::cast_from((w2 >> lane32) & 1u32) * two - one;
        let s3 = F::cast_from((w3 >> lane32) & 1u32) * two - one;

        let base = input_base + b * 128 + lane;
        let x0 = input[base];
        let x1 = input[base + 32];
        let x2 = input[base + 64];
        let x3 = input[base + 96];

        sum += scale * (s0 * x0 + s1 * x1 + s2 * x2 + s3 * x3);
    }

    // plane_sum operates per-warp (CUDA warp shuffle is per-warp)
    sum = plane_sum(sum);

    if UNIT_POS_X == 0u32 {
        output[row * d_out_us + j] = sum;
    }
}

/// Launch the fused Q1_0_g128 dequant + GEMV kernel.
///
/// - packed_words: CubeTensor [n_blocks * 4], dtype I32 → reinterpreted as U32
/// - scales: CubeTensor [n_blocks], dtype F32
/// - input: CubeTensor [n_rows * d_in], dtype F32
/// - Returns: CubeTensor [n_rows, d_out], dtype F32
pub fn launch<R: CubeRuntime>(
    packed_words: CubeTensor<R>,
    scales: CubeTensor<R>,
    input: CubeTensor<R>,
    d_in: usize,
    d_out: usize,
    n_rows: usize,
) -> CubeTensor<R> {
    let blocks_per_row = d_in / 128;
    assert!(d_in % 128 == 0, "d_in ({d_in}) must be a multiple of 128");

    let client = input.client.clone();

    let output = empty_device_dtype(
        client.clone(),
        input.device.clone(),
        Shape::from([n_rows * d_out]),
        DType::F32,
    );

    // Reinterpret i32 packed words as u32 (same bit pattern, zero copy)
    let packed_u32 = CubeTensor::new(
        packed_words.client,
        packed_words.handle,
        *packed_words.meta,
        packed_words.device,
        DType::U32,
    );

    let warps_per_block: u32 = 4;
    let cube_dim = CubeDim::new_2d(32, warps_per_block);
    let blocks_x = (d_out as u32 + warps_per_block - 1) / warps_per_block;
    let cube_count = CubeCount::new_2d(blocks_x, n_rows as u32);

    unsafe {
        q1_fused_gemv::launch_unchecked::<f32, R>(
            &client,
            cube_count,
            cube_dim,
            packed_u32.into_tensor_arg(),
            scales.into_tensor_arg(),
            input.into_tensor_arg(),
            output.clone().into_tensor_arg(),
            blocks_per_row as u32,
            warps_per_block,
            d_in as u32,
            d_out as u32,
        );
    }

    output
}
