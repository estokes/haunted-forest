use anyhow::{ensure, Result};
use half::f16;

/// Block size for Q1_0_g128: 128 weights per block.
const Q1_0_G128_BLOCK_SIZE: usize = 128;
/// Byte size of one Q1_0_g128 block: 2 (f16 scale) + 16 (128 bits packed).
const Q1_0_G128_BLOCK_BYTES: usize = 18;

/// Block size for Q1_0: 32 weights per block.
const Q1_0_BLOCK_SIZE: usize = 32;
/// Byte size of one Q1_0 block: 2 (f16 scale) + 4 (32 bits packed).
const Q1_0_BLOCK_BYTES: usize = 6;

/// Dequantize Q1_0_g128 packed data directly to f16.
///
/// Since the scale is already f16 and weights are binary, this is lossless.
/// bit=1 → +scale, bit=0 → -scale (negation is just a sign-bit flip in f16).
pub fn dequantize_q1_0_g128_f16(data: &[u8], n_elements: usize) -> Result<Vec<f16>> {
    let n_blocks = (n_elements + Q1_0_G128_BLOCK_SIZE - 1) / Q1_0_G128_BLOCK_SIZE;
    ensure!(
        data.len() >= n_blocks * Q1_0_G128_BLOCK_BYTES,
        "Q1_0_g128: data too short: {} bytes for {} elements ({} blocks)",
        data.len(), n_elements, n_blocks,
    );

    let mut out = Vec::with_capacity(n_elements);
    for i in 0..n_blocks {
        let block = &data[i * Q1_0_G128_BLOCK_BYTES..];
        let scale = f16::from_le_bytes([block[0], block[1]]);
        let neg_scale = -scale;
        let bits = &block[2..Q1_0_G128_BLOCK_BYTES];

        let remaining = n_elements - out.len();
        let count = remaining.min(Q1_0_G128_BLOCK_SIZE);

        for j in 0..count {
            let bit = (bits[j / 8] >> (j % 8)) & 1;
            out.push(if bit != 0 { scale } else { neg_scale });
        }
    }

    Ok(out)
}

/// Dequantize Q1_0_g128 packed data into f32.
///
/// Each block: 2 bytes f16 scale, then 16 bytes of packed bits.
/// bit=1 → +scale, bit=0 → -scale
pub fn dequantize_q1_0_g128(data: &[u8], n_elements: usize) -> Result<Vec<f32>> {
    let n_blocks = (n_elements + Q1_0_G128_BLOCK_SIZE - 1) / Q1_0_G128_BLOCK_SIZE;
    ensure!(
        data.len() >= n_blocks * Q1_0_G128_BLOCK_BYTES,
        "Q1_0_g128: data too short: {} bytes for {} elements ({} blocks)",
        data.len(), n_elements, n_blocks,
    );

    let mut out = Vec::with_capacity(n_elements);
    for i in 0..n_blocks {
        let block = &data[i * Q1_0_G128_BLOCK_BYTES..];
        let scale = f16::from_le_bytes([block[0], block[1]]).to_f32();
        let neg_scale = -scale;
        let bits = &block[2..Q1_0_G128_BLOCK_BYTES];

        let remaining = n_elements - out.len();
        let count = remaining.min(Q1_0_G128_BLOCK_SIZE);

        for j in 0..count {
            let bit = (bits[j / 8] >> (j % 8)) & 1;
            out.push(if bit != 0 { scale } else { neg_scale });
        }
    }

    Ok(out)
}

/// Dequantize Q1_0 packed data into f32.
///
/// Each block: 2 bytes f16 scale, then 4 bytes of packed bits.
/// bit=1 → +scale, bit=0 → -scale
pub fn dequantize_q1_0(data: &[u8], n_elements: usize) -> Result<Vec<f32>> {
    let n_blocks = (n_elements + Q1_0_BLOCK_SIZE - 1) / Q1_0_BLOCK_SIZE;
    ensure!(
        data.len() >= n_blocks * Q1_0_BLOCK_BYTES,
        "Q1_0: data too short: {} bytes for {} elements ({} blocks)",
        data.len(), n_elements, n_blocks,
    );

    let mut out = Vec::with_capacity(n_elements);
    for i in 0..n_blocks {
        let block = &data[i * Q1_0_BLOCK_BYTES..];
        let scale = f16::from_le_bytes([block[0], block[1]]).to_f32();
        let neg_scale = -scale;
        let bits = &block[2..Q1_0_BLOCK_BYTES];

        let remaining = n_elements - out.len();
        let count = remaining.min(Q1_0_BLOCK_SIZE);

        for j in 0..count {
            let bit = (bits[j / 8] >> (j % 8)) & 1;
            out.push(if bit != 0 { scale } else { neg_scale });
        }
    }

    Ok(out)
}

/// Read raw f32 values from a byte slice.
pub fn read_f32_data(data: &[u8], n_elements: usize) -> Result<Vec<f32>> {
    ensure!(
        data.len() >= n_elements * 4,
        "F32: data too short: {} bytes for {} elements",
        data.len(), n_elements,
    );

    let mut out = Vec::with_capacity(n_elements);
    for i in 0..n_elements {
        let bytes = [data[i * 4], data[i * 4 + 1], data[i * 4 + 2], data[i * 4 + 3]];
        out.push(f32::from_le_bytes(bytes));
    }
    Ok(out)
}

/// Read raw f16 values and convert to f32.
pub fn read_f16_data(data: &[u8], n_elements: usize) -> Result<Vec<f32>> {
    ensure!(
        data.len() >= n_elements * 2,
        "F16: data too short: {} bytes for {} elements",
        data.len(), n_elements,
    );

    let mut out = Vec::with_capacity(n_elements);
    for i in 0..n_elements {
        let val = f16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
        out.push(val.to_f32());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dequantize_q1_0_g128_basic() {
        // Build a single block: scale=1.0, all bits set to 1
        let scale_bytes = f16::from_f32(1.0).to_le_bytes();
        let mut block = vec![scale_bytes[0], scale_bytes[1]];
        block.extend_from_slice(&[0xFF; 16]); // all 128 bits = 1

        let out = dequantize_q1_0_g128(&block, 128).unwrap();
        assert_eq!(out.len(), 128);
        for &v in &out {
            assert_eq!(v, 1.0);
        }
    }

    #[test]
    fn dequantize_q1_0_g128_alternating() {
        // scale=2.0, alternating bits: 0b01010101
        let scale_bytes = f16::from_f32(2.0).to_le_bytes();
        let mut block = vec![scale_bytes[0], scale_bytes[1]];
        block.extend_from_slice(&[0x55; 16]); // 01010101 pattern

        let out = dequantize_q1_0_g128(&block, 128).unwrap();
        assert_eq!(out.len(), 128);
        // bit 0 = 1 (+2), bit 1 = 0 (-2), bit 2 = 1 (+2), ...
        for (j, &v) in out.iter().enumerate() {
            let expected = if j % 2 == 0 { 2.0 } else { -2.0 };
            assert_eq!(v, expected, "mismatch at index {j}");
        }
    }

    #[test]
    fn dequantize_q1_0_g128_zeros() {
        // scale=3.0, all bits = 0
        let scale_bytes = f16::from_f32(3.0).to_le_bytes();
        let mut block = vec![scale_bytes[0], scale_bytes[1]];
        block.extend_from_slice(&[0x00; 16]);

        let out = dequantize_q1_0_g128(&block, 128).unwrap();
        for &v in &out {
            assert_eq!(v, -3.0);
        }
    }
}
