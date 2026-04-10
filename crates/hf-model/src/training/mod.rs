mod ste;
mod model;
mod convert;
pub mod activations;
pub mod forward;
pub mod backward;
pub mod optimizer;
pub mod trainer;

pub use ste::{quantize_dequantize_q1_g128, SteLinear};
pub use model::{Qwen3TrainBlock, Qwen3TrainModel};
pub use convert::{inference_to_train, train_to_inference};
pub use forward::forward_with_activations;
pub use backward::{cross_entropy_loss_and_grad, backward_head_and_norm, backward_one_layer, LayerGradients};
pub use optimizer::AdamConfig;
pub use trainer::ManualTrainer;
