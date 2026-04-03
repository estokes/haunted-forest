// hf-model: 1-bit Qwen3 model implementation
//
// Will contain:
// - GGUF file parsing and Q1_0_G128 format support
// - BitLinear layer (binary weights with STE training)
// - Qwen3 transformer architecture
// - Inference pipeline (autoregressive generation, KV cache)
// - Training pipeline (STE-based fine-tuning)
