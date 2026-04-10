use burn::tensor::{Int, Tensor, TensorData};

use crate::B;
use crate::fused_rope::FusedRoPE;
use crate::qwen3::Qwen3Model;
use crate::tokenizer::Tokenizer;
use super::backward::{cross_entropy_loss_and_grad, backward_head_and_norm, backward_one_layer};
use super::forward::forward_with_activations;
use super::optimizer::{AdamConfig, LayerOptState, adam_step_projection, init_layer_opt_state};

/// Manual training system for 1-bit quantized Qwen3.
///
/// Weights stay packed 1-bit on GPU (~240 MB for 1.7B).
/// Shadow weights + Adam states live on CPU (~10 GB for 1.7B).
/// Backward and optimizer are interleaved per-layer: only one layer's
/// gradients exist in GPU memory at a time.
pub struct ManualTrainer {
    pub model: Qwen3Model,
    pub rope: FusedRoPE,
    pub tokenizer: Tokenizer,
    layer_states: Vec<LayerOptState>,
    adam_cfg: AdamConfig,
    step: usize,
}

impl ManualTrainer {
    /// Initialize trainer from a loaded inference model.
    pub fn new(
        model: Qwen3Model,
        rope: FusedRoPE,
        tokenizer: Tokenizer,
        adam_cfg: AdamConfig,
    ) -> Self {
        let n_layers = model.config.n_layers;
        let mut layer_states = Vec::with_capacity(n_layers);

        for i in 0..n_layers {
            let layer = &model.layers[i];
            let state = init_layer_opt_state(
                &layer.attn.q_proj,
                &layer.attn.k_proj,
                &layer.attn.v_proj,
                &layer.attn.o_proj,
                &layer.ffn_gate,
                &layer.ffn_up,
                &layer.ffn_down,
            );
            layer_states.push(state);
        }

        ManualTrainer { model, rope, tokenizer, layer_states, adam_cfg, step: 0 }
    }

    /// Run one training step: forward → loss → backward+optimizer interleaved.
    ///
    /// Backward and optimizer are interleaved per-layer: compute one layer's
    /// gradients, apply Adam, re-quantize, then move to the next layer.
    /// Only one layer's gradients exist in GPU memory at a time.
    pub fn train_step(&mut self, token_ids: &[u32]) -> f32 {
        let device = burn::backend::cuda::CudaDevice::default();
        let cfg = self.model.config.clone();

        // Forward
        let token_tensor = Tensor::<B, 2, Int>::from_data(
            TensorData::new(
                token_ids.iter().map(|&t| t as i64).collect::<Vec<_>>(),
                [1, token_ids.len()],
            ),
            &device,
        );
        let (logits, acts) = forward_with_activations(&self.model, token_tensor, &self.rope);

        // Loss
        let (loss, grad_logits) = cross_entropy_loss_and_grad(
            logits, token_ids, cfg.vocab_size,
        );

        // Backward through lm_head + final_norm
        let (mut grad, _grad_w_lm) = backward_head_and_norm(
            grad_logits, &self.model, &acts, &cfg,
        );
        // TODO: Adam step for lm_head weight gradient

        // Backward through layers in reverse, interleaved with optimizer
        self.step += 1;
        let t = self.step;

        for i in (0..cfg.n_layers).rev() {
            // Compute this layer's gradients
            let (lg, grad_input) = backward_one_layer(
                grad, &acts.layers[i], &self.model.layers[i], &self.rope, &cfg,
            );

            // Apply Adam + re-quantize immediately (frees gradient memory)
            let layer = &mut self.model.layers[i];
            let state = &mut self.layer_states[i];

            adam_step_projection(&mut state.q_proj.0, &mut state.q_proj.1,
                lg.q_proj, &mut layer.attn.q_proj, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.k_proj.0, &mut state.k_proj.1,
                lg.k_proj, &mut layer.attn.k_proj, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.v_proj.0, &mut state.v_proj.1,
                lg.v_proj, &mut layer.attn.v_proj, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.o_proj.0, &mut state.o_proj.1,
                lg.o_proj, &mut layer.attn.o_proj, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.ffn_gate.0, &mut state.ffn_gate.1,
                lg.ffn_gate, &mut layer.ffn_gate, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.ffn_up.0, &mut state.ffn_up.1,
                lg.ffn_up, &mut layer.ffn_up, &self.adam_cfg, t, &device);
            adam_step_projection(&mut state.ffn_down.0, &mut state.ffn_down.1,
                lg.ffn_down, &mut layer.ffn_down, &self.adam_cfg, t, &device);

            grad = grad_input;
        }

        loss
    }
}
