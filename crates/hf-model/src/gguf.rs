use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use anyhow::{bail, ensure, Context, Result};
use memmap2::Mmap;

const GGUF_MAGIC: u32 = 0x46554747; // "GGUF" little-endian
const GGUF_VERSION: u32 = 3;
const GGUF_DEFAULT_ALIGNMENT: u64 = 32;

// GGUF metadata value types
const TYPE_U8: u32 = 0;
const TYPE_I8: u32 = 1;
const TYPE_U16: u32 = 2;
const TYPE_I16: u32 = 3;
const TYPE_U32: u32 = 4;
const TYPE_I32: u32 = 5;
const TYPE_F32: u32 = 6;
const TYPE_BOOL: u32 = 7;
const TYPE_STRING: u32 = 8;
const TYPE_ARRAY: u32 = 9;
const TYPE_U64: u32 = 10;
const TYPE_I64: u32 = 11;
const TYPE_F64: u32 = 12;

// GGML tensor types we care about
pub const GGML_TYPE_F32: u32 = 0;
pub const GGML_TYPE_F16: u32 = 1;
pub const GGML_TYPE_Q1_0_G128: u32 = 41;
pub const GGML_TYPE_Q1_0: u32 = 42;

#[derive(Debug, Clone)]
pub enum GgufValue {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    Bool(bool),
    String(String),
    U64(u64),
    I64(i64),
    F64(f64),
    ArrayU8(Vec<u8>),
    ArrayI8(Vec<i8>),
    ArrayU16(Vec<u16>),
    ArrayI16(Vec<i16>),
    ArrayU32(Vec<u32>),
    ArrayI32(Vec<i32>),
    ArrayF32(Vec<f32>),
    ArrayBool(Vec<bool>),
    ArrayString(Vec<String>),
    ArrayU64(Vec<u64>),
    ArrayI64(Vec<i64>),
    ArrayF64(Vec<f64>),
}

impl GgufValue {
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            GgufValue::U32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            GgufValue::F32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            GgufValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_string_array(&self) -> Option<&[String]> {
        match self {
            GgufValue::ArrayString(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_i32_array(&self) -> Option<&[i32]> {
        match self {
            GgufValue::ArrayI32(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            GgufValue::U64(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TensorInfo {
    pub name: String,
    pub dims: Vec<u64>,
    pub dtype: u32,
    pub offset: u64, // offset from start of data section
}

impl TensorInfo {
    pub fn n_elements(&self) -> u64 {
        self.dims.iter().product::<u64>().max(1)
    }

    pub fn data_size(&self) -> u64 {
        let n = self.n_elements();
        match self.dtype {
            GGML_TYPE_F32 => n * 4,
            GGML_TYPE_F16 => n * 2,
            GGML_TYPE_Q1_0_G128 => {
                // 128 elements per block, each block = 18 bytes (2 f16 + 16 bits)
                let n_blocks = (n + 127) / 128;
                n_blocks * 18
            }
            GGML_TYPE_Q1_0 => {
                // 32 elements per block, each block = 6 bytes (2 f16 + 4 bits)
                let n_blocks = (n + 31) / 32;
                n_blocks * 6
            }
            _ => 0,
        }
    }
}

pub struct GgufFile {
    pub metadata: HashMap<String, GgufValue>,
    pub tensors: Vec<TensorInfo>,
    pub data_offset: u64,
    mmap: Mmap,
}

impl GgufFile {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("failed to open GGUF file: {}", path.display()))?;
        let mmap = unsafe { Mmap::map(&file) }
            .with_context(|| "failed to mmap GGUF file")?;

        let mut cursor = io::Cursor::new(&mmap[..]);
        let header = Header::read(&mut cursor)?;

        let mut metadata = HashMap::with_capacity(header.kv_count as usize);
        for _ in 0..header.kv_count {
            let (key, value) = read_kv_pair(&mut cursor)?;
            metadata.insert(key, value);
        }

        let mut tensors = Vec::with_capacity(header.tensor_count as usize);
        for _ in 0..header.tensor_count {
            tensors.push(read_tensor_info(&mut cursor)?);
        }

        // Data section starts at alignment boundary after all metadata + tensor info
        let alignment = metadata
            .get("general.alignment")
            .and_then(|v| v.as_u32())
            .map(|v| v as u64)
            .unwrap_or(GGUF_DEFAULT_ALIGNMENT);

        let pos = cursor.position();
        let data_offset = align_to(pos, alignment);

        Ok(GgufFile {
            metadata,
            tensors,
            data_offset,
            mmap,
        })
    }

    pub fn tensor_data(&self, info: &TensorInfo) -> &[u8] {
        let start = (self.data_offset + info.offset) as usize;
        let size = info.data_size() as usize;
        &self.mmap[start..start + size]
    }

    pub fn get_tensor(&self, name: &str) -> Option<&TensorInfo> {
        self.tensors.iter().find(|t| t.name == name)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).and_then(|v| v.as_str())
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.metadata.get(key).and_then(|v| v.as_u32())
    }

    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.metadata.get(key).and_then(|v| v.as_f32())
    }
}

fn align_to(pos: u64, alignment: u64) -> u64 {
    (pos + alignment - 1) / alignment * alignment
}

struct Header {
    tensor_count: u64,
    kv_count: u64,
}

impl Header {
    fn read<R: Read>(r: &mut R) -> Result<Self> {
        let magic = read_u32(r)?;
        ensure!(magic == GGUF_MAGIC, "not a GGUF file (bad magic: 0x{magic:08x})");

        let version = read_u32(r)?;
        ensure!(version == GGUF_VERSION, "unsupported GGUF version {version} (expected {GGUF_VERSION})");

        let tensor_count = read_u64(r)?;
        let kv_count = read_u64(r)?;

        Ok(Header { tensor_count, kv_count })
    }
}

fn read_kv_pair<R: Read>(r: &mut R) -> Result<(String, GgufValue)> {
    let key = read_string(r)?;
    let type_id = read_u32(r)?;
    let value = read_value(r, type_id)?;
    Ok((key, value))
}

fn read_value<R: Read>(r: &mut R, type_id: u32) -> Result<GgufValue> {
    let v = match type_id {
        TYPE_U8 => GgufValue::U8(read_u8(r)?),
        TYPE_I8 => GgufValue::I8(read_u8(r)? as i8),
        TYPE_U16 => GgufValue::U16(read_u16(r)?),
        TYPE_I16 => GgufValue::I16(read_u16(r)? as i16),
        TYPE_U32 => GgufValue::U32(read_u32(r)?),
        TYPE_I32 => GgufValue::I32(read_u32(r)? as i32),
        TYPE_F32 => GgufValue::F32(f32::from_bits(read_u32(r)?)),
        TYPE_BOOL => GgufValue::Bool(read_u8(r)? != 0),
        TYPE_STRING => GgufValue::String(read_string(r)?),
        TYPE_U64 => GgufValue::U64(read_u64(r)?),
        TYPE_I64 => GgufValue::I64(read_u64(r)? as i64),
        TYPE_F64 => GgufValue::F64(f64::from_bits(read_u64(r)?)),
        TYPE_ARRAY => read_array(r)?,
        other => bail!("unknown GGUF value type: {other}"),
    };
    Ok(v)
}

fn read_array<R: Read>(r: &mut R) -> Result<GgufValue> {
    let elem_type = read_u32(r)?;
    let count = read_u64(r)? as usize;

    let v = match elem_type {
        TYPE_U8 => {
            let mut buf = vec![0u8; count];
            r.read_exact(&mut buf)?;
            GgufValue::ArrayU8(buf)
        }
        TYPE_I8 => {
            let mut buf = vec![0u8; count];
            r.read_exact(&mut buf)?;
            GgufValue::ArrayI8(buf.into_iter().map(|b| b as i8).collect())
        }
        TYPE_U16 => GgufValue::ArrayU16(read_vec(r, count, read_u16)?),
        TYPE_I16 => GgufValue::ArrayI16(read_vec(r, count, |r| Ok(read_u16(r)? as i16))?),
        TYPE_U32 => GgufValue::ArrayU32(read_vec(r, count, read_u32)?),
        TYPE_I32 => GgufValue::ArrayI32(read_vec(r, count, |r| Ok(read_u32(r)? as i32))?),
        TYPE_F32 => GgufValue::ArrayF32(read_vec(r, count, |r| Ok(f32::from_bits(read_u32(r)?)))?),
        TYPE_BOOL => GgufValue::ArrayBool(read_vec(r, count, |r| Ok(read_u8(r)? != 0))?),
        TYPE_STRING => GgufValue::ArrayString(read_vec(r, count, read_string)?),
        TYPE_U64 => GgufValue::ArrayU64(read_vec(r, count, read_u64)?),
        TYPE_I64 => GgufValue::ArrayI64(read_vec(r, count, |r| Ok(read_u64(r)? as i64))?),
        TYPE_F64 => GgufValue::ArrayF64(read_vec(r, count, |r| Ok(f64::from_bits(read_u64(r)?)))?),
        other => bail!("unknown GGUF array element type: {other}"),
    };
    Ok(v)
}

fn read_tensor_info<R: Read>(r: &mut R) -> Result<TensorInfo> {
    let name = read_string(r)?;
    let n_dims = read_u32(r)? as usize;
    let mut dims = Vec::with_capacity(n_dims);
    for _ in 0..n_dims {
        dims.push(read_u64(r)?);
    }
    let dtype = read_u32(r)?;
    let offset = read_u64(r)?;
    Ok(TensorInfo { name, dims, dtype, offset })
}

fn read_vec<R: Read, T>(r: &mut R, count: usize, read_fn: impl Fn(&mut R) -> Result<T>) -> Result<Vec<T>> {
    let mut v = Vec::with_capacity(count);
    for _ in 0..count {
        v.push(read_fn(r)?);
    }
    Ok(v)
}

// Primitive readers (little-endian)

fn read_u8<R: Read>(r: &mut R) -> Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_u16<R: Read>(r: &mut R) -> Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32<R: Read>(r: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64<R: Read>(r: &mut R) -> Result<u64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_string<R: Read>(r: &mut R) -> Result<String> {
    let len = read_u64(r)? as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    String::from_utf8(buf).context("invalid UTF-8 in GGUF string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bonsai_1_7b() {
        let path = Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }

        let gguf = GgufFile::open(path).expect("failed to parse GGUF");

        assert_eq!(gguf.get_str("general.architecture"), Some("qwen3"));
        assert_eq!(gguf.get_u32("qwen3.block_count"), Some(28));
        assert_eq!(gguf.get_u32("qwen3.embedding_length"), Some(2048));
        assert_eq!(gguf.get_u32("qwen3.feed_forward_length"), Some(6144));
        assert_eq!(gguf.get_u32("qwen3.attention.head_count"), Some(16));
        assert_eq!(gguf.get_u32("qwen3.attention.head_count_kv"), Some(8));

        assert_eq!(gguf.tensors.len(), 310);

        // Check a known tensor
        let embd = gguf.get_tensor("token_embd.weight").expect("missing token_embd");
        assert_eq!(embd.dims, vec![2048, 151669]);
        assert_eq!(embd.dtype, GGML_TYPE_Q1_0_G128);

        let norm = gguf.get_tensor("output_norm.weight").expect("missing output_norm");
        assert_eq!(norm.dims, vec![2048]);
        assert_eq!(norm.dtype, GGML_TYPE_F32);

        println!("GGUF loaded: {} tensors, data_offset={}", gguf.tensors.len(), gguf.data_offset);
        for t in gguf.tensors.iter().take(5) {
            println!("  {}: dims={:?} dtype={} offset={}", t.name, t.dims, t.dtype, t.offset);
        }
    }
}
