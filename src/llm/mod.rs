mod thread;
pub mod types;

use std::sync::atomic::{AtomicU64, Ordering};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};

use types::*;

/// Bevy resource for communicating with the background LLM thread.
#[derive(Resource)]
pub struct LlmBridge {
    tx: Sender<LlmRequest>,
    rx: Receiver<LlmResponse>,
    #[allow(dead_code)]
    next_id: AtomicU64,
}

impl LlmBridge {
    fn start() -> Self {
        let (req_tx, req_rx) = unbounded::<LlmRequest>();
        let (resp_tx, resp_rx) = unbounded::<LlmResponse>();

        std::thread::Builder::new()
            .name("llm-worker".into())
            .spawn(move || thread::run(req_rx, resp_tx))
            .expect("failed to spawn LLM thread");

        Self {
            tx: req_tx,
            rx: resp_rx,
            next_id: AtomicU64::new(1),
        }
    }

    /// Submit a generation request. Returns the correlation ID.
    #[allow(dead_code)]
    pub fn generate(
        &self,
        model: ModelId,
        prompt: String,
        max_tokens: usize,
        temperature: f64,
    ) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let _ = self.tx.send(LlmRequest::Generate {
            id,
            model,
            prompt,
            max_tokens,
            temperature,
        });
        id
    }

    /// Cancel an in-flight generation.
    #[allow(dead_code)]
    pub fn cancel(&self, id: u64) {
        let _ = self.tx.send(LlmRequest::Cancel { id });
    }

    /// Request a model swap.
    #[allow(dead_code)]
    pub fn swap_model(&self, model: ModelId) {
        let _ = self.tx.send(LlmRequest::SwapModel(model));
    }

    /// Drain all pending responses. Call once per frame.
    pub fn drain_responses(&self) -> impl Iterator<Item = LlmResponse> + '_ {
        self.rx.try_iter()
    }
}

impl Drop for LlmBridge {
    fn drop(&mut self) {
        let _ = self.tx.send(LlmRequest::Shutdown);
    }
}

pub struct LlmPlugin;

impl Plugin for LlmPlugin {
    fn build(&self, app: &mut App) {
        let bridge = LlmBridge::start();
        app.insert_resource(bridge)
            .add_systems(Update, drain_llm_responses);
    }
}

fn drain_llm_responses(bridge: Res<LlmBridge>) {
    for response in bridge.drain_responses() {
        match response {
            LlmResponse::Ready => log::info!("LLM thread ready"),
            LlmResponse::Token { id, text } => {
                log::trace!("LLM [{id}]: {text}");
            }
            LlmResponse::Done { id } => {
                log::info!("LLM [{id}]: generation complete");
            }
            LlmResponse::Error { id, message } => {
                log::warn!("LLM [{id}]: {message}");
            }
            LlmResponse::ModelSwapped(model) => {
                log::info!("LLM: swapped to {model:?}");
            }
        }
    }
}
