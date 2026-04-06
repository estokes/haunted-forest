use std::path::Path;

use anyhow::{bail, Context, Result};
use burn::module::Param;
use burn::nn::RotaryEncoding;
use burn::tensor::{Shape, Tensor, TensorData};

use crate::B;
use crate::bit_linear::BitLinear;
use crate::gguf::{GgufFile, TensorInfo, GGML_TYPE_F32, GGML_TYPE_Q1_0, GGML_TYPE_Q1_0_G128};
use crate::quant;
use crate::qwen3::{ModelConfig, Qwen3Model};
use crate::tokenizer::Tokenizer;

type Device = burn::backend::cuda::CudaDevice;

/// Load a Bonsai GGUF model, returning the model, tokenizer, and RoPE module.
pub fn load_model(
    path: &Path,
    device: &Device,
) -> Result<(Qwen3Model, Tokenizer, RotaryEncoding<B>)> {
    log::info!("loading GGUF: {}", path.display());

    let gguf = GgufFile::open(path)?;
    let config = ModelConfig::from_gguf(&gguf)?;
    let tokenizer = Tokenizer::from_gguf(&gguf)?;

    log::info!(
        "model: {}L d={} ff={} q_heads={} kv_heads={} d_head={} vocab={}",
        config.n_layers, config.d_model, config.d_ff,
        config.n_q_heads, config.n_kv_heads, config.d_head, config.vocab_size,
    );

    let mut model = Qwen3Model::new(&config, device);
    let rope = model.create_rope(device);

    load_weights(&gguf, &config, &mut model, device)?;

    log::info!("model loaded successfully");

    Ok((model, tokenizer, rope))
}

fn load_weights(
    gguf: &GgufFile,
    cfg: &ModelConfig,
    model: &mut Qwen3Model,
    device: &Device,
) -> Result<()> {
    // Global tensors (stay in f32)
    load_embedding(gguf, "token_embd.weight", &mut model.embedding, device)?;
    load_norm(gguf, "output_norm.weight", &mut model.final_norm, device)?;

    // lm_head: use output.weight if present, otherwise tie with embedding
    if gguf.get_tensor("output.weight").is_some() {
        load_linear_f32(gguf, "output.weight", &mut model.lm_head, device)?;
    } else {
        let embd_weight = model.embedding.weight.val();
        let lm_weight = embd_weight.transpose();
        model.lm_head.weight = Param::from_tensor(lm_weight);
        model.lm_head.bias = None;
        log::info!("lm_head tied with token_embd");
    }

    // Per-layer tensors
    for i in 0..cfg.n_layers {
        let layer = &mut model.layers[i];
        let pfx = format!("blk.{i}");

        // Norms stay in f32
        load_norm(gguf, &format!("{pfx}.attn_norm.weight"), &mut layer.attn_norm, device)?;
        load_norm(gguf, &format!("{pfx}.attn_q_norm.weight"), &mut layer.attn.q_norm, device)?;
        load_norm(gguf, &format!("{pfx}.attn_k_norm.weight"), &mut layer.attn.k_norm, device)?;
        load_norm(gguf, &format!("{pfx}.ffn_norm.weight"), &mut layer.ffn_norm, device)?;

        // Projections: packed Q1_0_g128 bytes, uploaded to GPU
        layer.attn.q_proj = load_bit_linear(gguf, &format!("{pfx}.attn_q.weight"), device)?;
        layer.attn.k_proj = load_bit_linear(gguf, &format!("{pfx}.attn_k.weight"), device)?;
        layer.attn.v_proj = load_bit_linear(gguf, &format!("{pfx}.attn_v.weight"), device)?;
        layer.attn.o_proj = load_bit_linear(gguf, &format!("{pfx}.attn_output.weight"), device)?;
        layer.ffn_gate = load_bit_linear(gguf, &format!("{pfx}.ffn_gate.weight"), device)?;
        layer.ffn_up = load_bit_linear(gguf, &format!("{pfx}.ffn_up.weight"), device)?;
        layer.ffn_down = load_bit_linear(gguf, &format!("{pfx}.ffn_down.weight"), device)?;

        if (i + 1) % 4 == 0 || i + 1 == cfg.n_layers {
            log::info!("loaded layer {}/{}", i + 1, cfg.n_layers);
        }
    }

    Ok(())
}

/// Load packed Q1_0_g128 bytes into a GPU-resident BitLinear.
fn load_bit_linear(gguf: &GgufFile, name: &str, device: &Device) -> Result<BitLinear> {
    let info = gguf.get_tensor(name)
        .with_context(|| format!("missing tensor: {name}"))?;

    let data = gguf.tensor_data(info);
    let n_elements = info.n_elements() as usize;
    let ne0 = info.dims[0] as usize;
    let ne1 = info.dims[1] as usize;

    Ok(BitLinear::from_packed(data, n_elements, [ne0, ne1], device))
}

/// Dequantize a GGUF tensor to f32 Vec (used for embedding and lm_head).
fn dequantize_tensor(gguf: &GgufFile, info: &TensorInfo) -> Result<Vec<f32>> {
    let data = gguf.tensor_data(info);
    let n_elements = info.n_elements() as usize;

    match info.dtype {
        GGML_TYPE_F32 => quant::read_f32_data(data, n_elements),
        GGML_TYPE_Q1_0_G128 => quant::dequantize_q1_0_g128(data, n_elements),
        GGML_TYPE_Q1_0 => quant::dequantize_q1_0(data, n_elements),
        other => bail!("unsupported GGML type {} for tensor {}", other, info.name),
    }
}

/// Load a 2D weight tensor into a Linear layer (f32, for embedding/lm_head).
fn load_linear_f32(
    gguf: &GgufFile,
    name: &str,
    linear: &mut burn::nn::Linear<B>,
    device: &Device,
) -> Result<()> {
    let info = gguf.get_tensor(name)
        .with_context(|| format!("missing tensor: {name}"))?;

    let f32_data = dequantize_tensor(gguf, info)?;

    let ne0 = info.dims[0] as usize;
    let ne1 = info.dims[1] as usize;
    let tensor_data = TensorData::new(f32_data, Shape::from([ne1, ne0]));
    let tensor: Tensor<B, 2> = Tensor::from_data(tensor_data, device);
    let tensor = tensor.transpose();

    linear.weight = Param::from_tensor(tensor);
    linear.bias = None;

    Ok(())
}

/// Load a 1D weight tensor into a FusedRmsNorm layer (the gamma parameter).
fn load_norm(
    gguf: &GgufFile,
    name: &str,
    norm: &mut crate::fused_rms_norm::FusedRmsNorm,
    device: &Device,
) -> Result<()> {
    let info = gguf.get_tensor(name)
        .with_context(|| format!("missing tensor: {name}"))?;

    let f32_data = dequantize_tensor(gguf, info)?;

    let shape: Vec<usize> = info.dims.iter().map(|&d| d as usize).collect();
    let tensor_data = TensorData::new(f32_data, Shape::from(shape));
    let tensor: Tensor<B, 1> = Tensor::from_data(tensor_data, device);

    norm.gamma = tensor;

    Ok(())
}

/// Load a 2D weight tensor into an Embedding layer.
fn load_embedding(
    gguf: &GgufFile,
    name: &str,
    embd: &mut burn::nn::Embedding<B>,
    device: &Device,
) -> Result<()> {
    let info = gguf.get_tensor(name)
        .with_context(|| format!("missing tensor: {name}"))?;

    let f32_data = dequantize_tensor(gguf, info)?;

    let ne0 = info.dims[0] as usize;
    let ne1 = info.dims[1] as usize;
    let tensor_data = TensorData::new(f32_data, Shape::from([ne1, ne0]));
    let tensor: Tensor<B, 2> = Tensor::from_data(tensor_data, device);

    embd.weight = Param::from_tensor(tensor);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bonsai_1_7b_model() {
        let path = Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }

        let device = burn::backend::cuda::CudaDevice::default();
        let (model, tokenizer, _) = load_model(path, &device).unwrap();

        assert_eq!(model.config.n_layers, 28);
        assert_eq!(model.config.d_model, 2048);
        assert_eq!(tokenizer.vocab_size(), 151669);

        println!("Model loaded successfully!");
        println!("  layers: {}", model.config.n_layers);
        println!("  d_model: {}", model.config.d_model);
        println!("  vocab: {}", tokenizer.vocab_size());
    }

    #[test]
    fn generate_greedy() {
        use std::time::Instant;

        let path = Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }

        let device = burn::backend::cuda::CudaDevice::default();
        let (model, tokenizer, rope) = load_model(path, &device).unwrap();

        let prompt = "The capital of France is";
        let input_ids = tokenizer.encode(prompt);
        println!("prompt: {prompt:?}");
        println!("input_ids: {input_ids:?}");

        let mut state = crate::qwen3::InferenceState::new(model.config.n_layers);
        let params = crate::sample::SampleParams::greedy();

        // Prefill
        let t0 = Instant::now();
        let input_tensor = Tensor::<B, 2, burn::tensor::Int>::from_data(
            TensorData::new(input_ids.iter().map(|&id| id as i64).collect::<Vec<_>>(), [1, input_ids.len()]),
            &device,
        );

        let logits = model.forward(input_tensor, &rope, &mut state);
        let [_b, seq_len, _vocab] = logits.dims();
        let last_logits = logits.slice([0..1, (seq_len - 1)..seq_len, 0..model.config.vocab_size]);
        let last_logits = last_logits.reshape([model.config.vocab_size]);

        let mut next_token = crate::sample::sample_token(last_logits, &params);
        // sample_token calls into_scalar which forces GPU sync
        let prefill_ms = t0.elapsed().as_millis();

        let mut output_tokens = vec![next_token];
        print!("{prompt}");
        print!("{}", tokenizer.decode(&[next_token]));

        // Decode loop with per-token timing
        let mut decode_times_ms = Vec::new();
        for _ in 1..50 {
            let t_tok = Instant::now();
            let input = Tensor::<B, 2, burn::tensor::Int>::from_data(
                TensorData::new(vec![next_token as i64], [1, 1]),
                &device,
            );

            let logits = model.forward(input, &rope, &mut state);
            let logits = logits.reshape([model.config.vocab_size]);
            let next_token_new = crate::sample::sample_token(logits, &params);
            // sample_token forces GPU sync via into_scalar
            decode_times_ms.push(t_tok.elapsed().as_millis());

            print!("{}", tokenizer.decode(&[next_token_new]));
            output_tokens.push(next_token_new);

            if tokenizer.eos_id == Some(next_token_new) {
                break;
            }
            let _ = std::mem::replace(&mut next_token, next_token_new);
        }
        println!();

        let n_decode = decode_times_ms.len();
        let total_decode_ms: u128 = decode_times_ms.iter().sum();
        let avg_ms = total_decode_ms as f64 / n_decode as f64;
        let tok_per_s = 1000.0 / avg_ms;

        println!("--- timing ---");
        println!("prefill ({} tokens): {}ms", input_ids.len(), prefill_ms);
        println!("decode: {} tokens in {}ms", n_decode, total_decode_ms);
        println!("  avg: {:.1}ms/token = {:.1} tok/s", avg_ms, tok_per_s);
        println!("  first 5 tokens: {:?}ms", &decode_times_ms[..5.min(n_decode)]);
        if n_decode > 5 {
            println!("  last 5 tokens: {:?}ms", &decode_times_ms[n_decode-5..]);
        }
    }
}
