pub mod attention;
pub mod bit_linear;
pub mod gguf;
pub mod quant;
pub mod qwen3;
pub mod sample;
pub mod tokenizer;

pub mod fused_gemv;
pub mod fused_rms_norm;

mod loader;

/// Concrete backend — all model types are monomorphized to CUDA.
pub type B = burn::backend::Cuda;

pub use loader::load_model;
pub use qwen3::{InferenceState, ModelConfig, Qwen3Model};
pub use sample::{SampleParams, sample_token};
pub use tokenizer::Tokenizer;
