#[allow(dead_code)]
/// Which character/role the model serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelId {
    /// 1B ship AI, always resident in VRAM.
    ShipAi,
    /// 3B faction character.
    FactionCharacter,
    /// 3B warden (separate fine-tune).
    Warden,
    /// 3B graphix coding assistant (separate fine-tune).
    GraphixAssistant,
}

#[allow(dead_code)]
/// Request from Bevy to the LLM background thread.
pub enum LlmRequest {
    Generate {
        id: u64,
        model: ModelId,
        prompt: String,
        max_tokens: usize,
        temperature: f64,
    },
    Cancel {
        id: u64,
    },
    SwapModel(ModelId),
    Shutdown,
}

/// Response from the LLM background thread to Bevy.
pub enum LlmResponse {
    Token { id: u64, text: String },
    Done { id: u64 },
    Error { id: u64, message: String },
    ModelSwapped(ModelId),
    Ready,
}
