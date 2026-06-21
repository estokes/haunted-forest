use std::io::BufRead;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::tokenizer::Tokenizer;

// ---- Types ----

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    fn as_str(self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

/// A plain text training example (for pre-training on world knowledge).
#[derive(Debug, Clone, Deserialize)]
pub struct TextExample {
    pub text: String,
}

// ---- Chat template formatting ----

impl Conversation {
    /// Format as Qwen3 chat template and tokenize.
    ///
    /// Tokenizes each message piecewise (matching inference-time behavior):
    /// ```text
    /// <|im_start|> + encode("system\n") + encode(content) + <|im_end|> + encode("\n")
    /// <|im_start|> + encode("user\n")   + encode(content) + <|im_end|> + encode("\n")
    /// ...
    /// ```
    pub fn to_token_ids(&self, tokenizer: &Tokenizer) -> Vec<u32> {
        let im_start = tokenizer.im_start_id.expect("tokenizer missing <|im_start|>");
        let im_end = tokenizer.im_end_id.expect("tokenizer missing <|im_end|>");
        let mut ids = Vec::new();
        for msg in &self.messages {
            ids.push(im_start);
            ids.extend(tokenizer.encode(&format!("{}\n", msg.role.as_str())));
            ids.extend(tokenizer.encode(&msg.content));
            ids.push(im_end);
            ids.extend(tokenizer.encode("\n"));
        }
        ids
    }

    /// Format and tokenize, returning a loss mask alongside token IDs.
    ///
    /// The mask is `true` for assistant content tokens and the
    /// `<|im_end|>` closing each assistant turn. System and user turns
    /// are context-only (mask = false).
    ///
    /// The mask aligns with the token IDs (same length). For training,
    /// the shifted mask `mask[1:]` tells you which target positions
    /// contribute to the loss.
    pub fn to_token_ids_with_mask(&self, tokenizer: &Tokenizer) -> (Vec<u32>, Vec<bool>) {
        let im_start = tokenizer.im_start_id.expect("tokenizer missing <|im_start|>");
        let im_end = tokenizer.im_end_id.expect("tokenizer missing <|im_end|>");
        let mut ids = Vec::new();
        let mut mask = Vec::new();

        for msg in &self.messages {
            let is_asst = msg.role == Role::Assistant;

            // <|im_start|>
            ids.push(im_start);
            mask.push(false);

            // "role\n"
            let role_ids = tokenizer.encode(&format!("{}\n", msg.role.as_str()));
            mask.extend(std::iter::repeat_n(false, role_ids.len()));
            ids.extend(role_ids);

            // content
            let content_ids = tokenizer.encode(&msg.content);
            mask.extend(std::iter::repeat_n(is_asst, content_ids.len()));
            ids.extend(content_ids);

            // <|im_end|>  — in loss for assistant turns so the model
            // learns to produce the stop token
            ids.push(im_end);
            mask.push(is_asst);

            // "\n"
            let nl_ids = tokenizer.encode("\n");
            mask.extend(std::iter::repeat_n(false, nl_ids.len()));
            ids.extend(nl_ids);
        }

        (ids, mask)
    }
}

// ---- JSONL loading ----

/// Load conversations from a JSONL file.
/// Each line: `{"messages": [{"role": "system", "content": "..."}, ...]}`
pub fn load_conversations(path: &Path) -> Result<Vec<Conversation>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    let reader = std::io::BufReader::new(file);
    let mut conversations = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("reading line {}", i + 1))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let conv: Conversation = serde_json::from_str(line)
            .with_context(|| format!("parsing line {}", i + 1))?;
        if conv.messages.is_empty() {
            bail!("line {}: conversation has no messages", i + 1);
        }
        conversations.push(conv);
    }
    Ok(conversations)
}

/// Load plain text examples from a JSONL file.
/// Each line: `{"text": "..."}`
pub fn load_plain_text(path: &Path) -> Result<Vec<String>> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    let reader = std::io::BufReader::new(file);
    let mut texts = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("reading line {}", i + 1))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let example: TextExample = serde_json::from_str(line)
            .with_context(|| format!("parsing line {}", i + 1))?;
        if !example.text.is_empty() {
            texts.push(example.text);
        }
    }
    Ok(texts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_conversation() -> Conversation {
        Conversation {
            messages: vec![
                Message {
                    role: Role::System,
                    content: "You are Kira, a merchant at Cygnus Station. Faction: Cygnus Trade Guild. Standing: 0.3 (warm).".into(),
                },
                Message {
                    role: Role::User,
                    content: "Got any helium-3?".into(),
                },
                Message {
                    role: Role::Assistant,
                    content: "Plenty. Price is fair for friends of the Guild.".into(),
                },
            ],
        }
    }

    #[test]
    fn tokenize_with_mask() {
        let path = std::path::Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }
        let gguf = crate::gguf::GgufFile::open(path).unwrap();
        let tok = Tokenizer::from_gguf(&gguf).unwrap();

        let conv = make_test_conversation();
        let ids = conv.to_token_ids(&tok);
        let (ids2, mask) = conv.to_token_ids_with_mask(&tok);

        assert_eq!(ids, ids2);
        assert_eq!(ids.len(), mask.len());

        // Count assistant tokens — should be the content + im_end
        let assistant_count = mask.iter().filter(|&&m| m).count();
        assert!(assistant_count > 1, "expected multiple assistant tokens, got {assistant_count}");

        // First token (system im_start) should be masked out
        assert!(!mask[0], "system im_start should be masked out");

        // Print token-by-token for debugging
        println!("total tokens: {}, assistant tokens: {}", ids.len(), assistant_count);
        for (i, (&id, &m)) in ids.iter().zip(mask.iter()).enumerate() {
            let tok_str = tok.decode_token(id);
            let marker = if m { " <-- TRAIN" } else { "" };
            println!("  [{i:>3}] {id:>6} {tok_str:?}{marker}");
        }

        // Decode to verify roundtrip
        let decoded = tok.decode(&ids);
        assert!(decoded.contains("Plenty. Price is fair"));
    }

    #[test]
    fn load_sample_conversations() {
        let path = std::path::Path::new("/home/eric/proj/haunted-forest/data/dialog/sample.jsonl");
        if !path.exists() {
            eprintln!("skipping: sample data not found");
            return;
        }
        let convs = load_conversations(path).unwrap();
        assert!(convs.len() >= 5, "expected at least 5 conversations, got {}", convs.len());

        // Verify all parse and tokenize
        let model_path = std::path::Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !model_path.exists() {
            eprintln!("skipping tokenize check: model not found");
            return;
        }
        let gguf = crate::gguf::GgufFile::open(model_path).unwrap();
        let tok = Tokenizer::from_gguf(&gguf).unwrap();

        for (i, conv) in convs.iter().enumerate() {
            let (ids, mask) = conv.to_token_ids_with_mask(&tok);
            let train_count = mask.iter().filter(|&&m| m).count();
            println!("conv {i}: {} tokens, {} trainable ({:.0}%)",
                ids.len(), train_count, 100.0 * train_count as f64 / ids.len() as f64);
            assert!(ids.len() >= 10, "conv {i} too short");
            assert!(train_count > 0, "conv {i} has no trainable tokens");
        }
    }

    #[test]
    fn load_jsonl() {
        let json = r#"{"messages":[{"role":"system","content":"You are a test NPC."},{"role":"user","content":"Hello"},{"role":"assistant","content":"Hi there."}]}"#;

        let dir = std::env::temp_dir().join("hf_model_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_conversations.jsonl");
        std::fs::write(&path, json).unwrap();

        let convs = load_conversations(&path).unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0].messages.len(), 3);
        assert_eq!(convs[0].messages[0].role, Role::System);
        assert_eq!(convs[0].messages[2].content, "Hi there.");

        std::fs::remove_file(&path).unwrap();
    }
}
