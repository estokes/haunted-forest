use burn::nn::{
    EmbeddingConfig, LinearConfig, RmsNormConfig,
};
use burn::prelude::*;

use crate::bit_linear::BitLinear;
use crate::fused_rms_norm::FusedRmsNorm;
use crate::qwen3::{LmHead, ModelConfig, Qwen3Block, Qwen3Model};
use super::model::{Qwen3TrainBlock, Qwen3TrainModel};
use super::ste::SteLinear;

/// Convert an inference model to a training model by dequantizing
/// all BitLinear weights to shadow weights in the training backend's float type.
pub fn inference_to_train<B: Backend>(
    inf: &Qwen3Model,
    device: &B::Device,
) -> Qwen3TrainModel<B> {
    let cfg = &inf.config;

    let embedding = copy_embedding::<B>(&inf.embedding, cfg.vocab_size, cfg.d_model, device);

    let layers = inf.layers.iter().map(|layer| {
        convert_block::<B>(layer, cfg, device)
    }).collect();

    let final_norm = copy_rms_norm::<B>(&inf.final_norm, cfg.d_model, cfg.rms_norm_eps, device);

    let lm_head = match &inf.lm_head {
        LmHead::Linear(lin) => copy_linear::<B>(lin, cfg.d_model, cfg.vocab_size, device),
        LmHead::BitLinear(bl) => dequant_to_linear::<B>(bl, device),
    };

    Qwen3TrainModel {
        embedding,
        layers,
        final_norm,
        lm_head,
        config: cfg.clone(),
    }
}

/// Convert a training model back to an inference model by requantizing
/// all SteLinear weights to BitLinear packed format.
///
/// Generic over the training backend — works with both `Cuda<f32>` and `Cuda<bf16>`.
pub fn train_to_inference<B: Backend>(
    train: &Qwen3TrainModel<B>,
    device: &burn::backend::cuda::CudaDevice,
) -> Qwen3Model {
    let cfg = &train.config;

    let mut model = Qwen3Model::new(cfg, device);

    // Copy embedding — go through TensorData which handles dtype conversion
    let emb_data = train.embedding.weight.val().into_data();
    model.embedding = EmbeddingConfig::new(cfg.vocab_size, cfg.d_model).init(device);
    model.embedding.weight = burn::module::Param::from_data(emb_data, device);

    // Convert layers
    for (i, train_layer) in train.layers.iter().enumerate() {
        let layer = &mut model.layers[i];

        layer.attn_norm = generic_to_fused_norm(&train_layer.attn_norm, cfg.d_model, cfg.rms_norm_eps, device);
        layer.attn.q_norm = generic_to_fused_norm(&train_layer.q_norm, cfg.d_head, cfg.rms_norm_eps, device);
        layer.attn.k_norm = generic_to_fused_norm(&train_layer.k_norm, cfg.d_head, cfg.rms_norm_eps, device);
        layer.ffn_norm = generic_to_fused_norm(&train_layer.ffn_norm, cfg.d_model, cfg.rms_norm_eps, device);

        layer.attn.q_proj = requantize_ste::<B>(&train_layer.q_proj, device);
        layer.attn.k_proj = requantize_ste::<B>(&train_layer.k_proj, device);
        layer.attn.v_proj = requantize_ste::<B>(&train_layer.v_proj, device);
        layer.attn.o_proj = requantize_ste::<B>(&train_layer.o_proj, device);

        layer.ffn_gate = requantize_ste::<B>(&train_layer.ffn_gate, device);
        layer.ffn_up = requantize_ste::<B>(&train_layer.ffn_up, device);
        layer.ffn_down = requantize_ste::<B>(&train_layer.ffn_down, device);
    }

    model.final_norm = generic_to_fused_norm(&train.final_norm, cfg.d_model, cfg.rms_norm_eps, device);

    // lm_head stays FP32 Linear
    let lm_data = train.lm_head.weight.val().into_data();
    model.lm_head = LmHead::Linear(LinearConfig::new(cfg.d_model, cfg.vocab_size).with_bias(false).init(device));
    if let LmHead::Linear(ref mut lin) = model.lm_head {
        lin.weight = burn::module::Param::from_data(lm_data, device);
    }

    model
}

// --- helpers ---

fn copy_embedding<B: Backend>(
    src: &burn::nn::Embedding<crate::B>,
    vocab: usize,
    d: usize,
    device: &B::Device,
) -> burn::nn::Embedding<B> {
    let data = src.weight.val().into_data();
    let mut emb = EmbeddingConfig::new(vocab, d).init(device);
    emb.weight = burn::module::Param::from_data(data, device);
    emb
}

fn copy_rms_norm<B: Backend>(
    src: &FusedRmsNorm,
    d: usize,
    eps: f64,
    device: &B::Device,
) -> burn::nn::RmsNorm<B> {
    let gamma_data = src.gamma.clone().into_data();
    let mut norm = RmsNormConfig::new(d).with_epsilon(eps).init(device);
    norm.gamma = burn::module::Param::from_data(gamma_data, device);
    norm
}

fn copy_linear<B: Backend>(
    src: &burn::nn::Linear<crate::B>,
    d_in: usize,
    d_out: usize,
    device: &B::Device,
) -> burn::nn::Linear<B> {
    let w_data = src.weight.val().into_data();
    let mut lin = LinearConfig::new(d_in, d_out).with_bias(false).init(device);
    lin.weight = burn::module::Param::from_data(w_data, device);
    lin
}

fn dequant_to_linear<B: Backend>(
    bl: &BitLinear,
    device: &B::Device,
) -> burn::nn::Linear<B> {
    // dequantize_weights returns [d_out, d_in] (GGUF/PyTorch convention).
    // burn's Linear stores weight as [d_input, d_output], so transpose.
    let w = bl.dequantize_weights(); // [d_out, d_in]
    let [d_out, d_in] = w.dims();
    let w_t = w.transpose(); // [d_in, d_out] — burn convention
    let w_data = w_t.into_data();
    let mut lin = LinearConfig::new(d_in, d_out).with_bias(false).init(device);
    lin.weight = burn::module::Param::from_data(w_data, device);
    lin
}

fn convert_block<B: Backend>(
    inf: &Qwen3Block,
    cfg: &ModelConfig,
    device: &B::Device,
) -> Qwen3TrainBlock<B> {
    Qwen3TrainBlock {
        attn_norm: copy_rms_norm::<B>(&inf.attn_norm, cfg.d_model, cfg.rms_norm_eps, device),
        q_proj: dequant_to_ste::<B>(&inf.attn.q_proj, device),
        k_proj: dequant_to_ste::<B>(&inf.attn.k_proj, device),
        v_proj: dequant_to_ste::<B>(&inf.attn.v_proj, device),
        o_proj: dequant_to_ste::<B>(&inf.attn.o_proj, device),
        q_norm: copy_rms_norm::<B>(&inf.attn.q_norm, cfg.d_head, cfg.rms_norm_eps, device),
        k_norm: copy_rms_norm::<B>(&inf.attn.k_norm, cfg.d_head, cfg.rms_norm_eps, device),
        ffn_norm: copy_rms_norm::<B>(&inf.ffn_norm, cfg.d_model, cfg.rms_norm_eps, device),
        ffn_gate: dequant_to_ste::<B>(&inf.ffn_gate, device),
        ffn_up: dequant_to_ste::<B>(&inf.ffn_up, device),
        ffn_down: dequant_to_ste::<B>(&inf.ffn_down, device),
        n_q_heads: cfg.n_q_heads,
        n_kv_heads: cfg.n_kv_heads,
        d_head: cfg.d_head,
    }
}

fn dequant_to_ste<B: Backend>(bl: &BitLinear, device: &B::Device) -> SteLinear<B> {
    let w = bl.dequantize_weights();
    let w_data = w.into_data();
    SteLinear::from_weight(Tensor::from_data(w_data, device))
}

/// Convert a generic-backend RmsNorm to our FusedRmsNorm (inference backend).
fn generic_to_fused_norm<B: Backend>(
    src: &burn::nn::RmsNorm<B>,
    d: usize,
    eps: f64,
    device: &burn::backend::cuda::CudaDevice,
) -> FusedRmsNorm {
    let gamma_data = src.gamma.val().into_data();
    FusedRmsNorm {
        gamma: Tensor::from_data(gamma_data, device),
        d,
        epsilon: eps,
    }
}

/// Convert SteLinear shadow weights to packed BitLinear.
/// Goes through TensorData for dtype-agnostic conversion (works with bf16 or f32).
fn requantize_ste<B: Backend>(
    ste: &SteLinear<B>,
    device: &burn::backend::cuda::CudaDevice,
) -> BitLinear {
    // Read shadow weights to CPU as f32 via TensorData
    let w_data = ste.weight.val().into_data();
    let w_f32: Tensor<crate::B, 2> = Tensor::from_data(w_data, device);
    BitLinear::from_f32_weights(w_f32, device)
}
