use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::gguf::GgufFile;

pub struct Tokenizer {
    /// Token ID → string
    vocab: Vec<String>,
    /// String → token ID (for single-token lookups)
    token_to_id: HashMap<String, u32>,
    /// BPE merge pairs: (left, right) → priority (lower = merge first)
    merges: HashMap<(String, String), u32>,
    /// Special token IDs
    pub bos_id: Option<u32>,
    pub eos_id: Option<u32>,
    pub pad_id: Option<u32>,
    /// Chat template tokens (Qwen3 `<|im_start|>` / `<|im_end|>`)
    pub im_start_id: Option<u32>,
    pub im_end_id: Option<u32>,
}

impl Tokenizer {
    pub fn from_gguf(gguf: &GgufFile) -> Result<Self> {
        let tokens = gguf
            .metadata
            .get("tokenizer.ggml.tokens")
            .and_then(|v| v.as_string_array())
            .context("missing tokenizer.ggml.tokens")?;

        let merge_strs = gguf
            .metadata
            .get("tokenizer.ggml.merges")
            .and_then(|v| v.as_string_array())
            .context("missing tokenizer.ggml.merges")?;

        let vocab: Vec<String> = tokens.to_vec();
        let mut token_to_id = HashMap::with_capacity(vocab.len());
        for (i, tok) in vocab.iter().enumerate() {
            token_to_id.insert(tok.clone(), i as u32);
        }

        let mut merges = HashMap::with_capacity(merge_strs.len());
        for (priority, m) in merge_strs.iter().enumerate() {
            let (left, right) = m
                .split_once(' ')
                .with_context(|| format!("invalid merge rule: {m:?}"))?;
            merges.insert((left.to_string(), right.to_string()), priority as u32);
        }

        let bos_id = gguf.get_u32("tokenizer.ggml.bos_token_id");
        let eos_id = gguf.get_u32("tokenizer.ggml.eos_token_id");
        let pad_id = gguf.get_u32("tokenizer.ggml.padding_token_id");

        let im_start_id = token_to_id.get("<|im_start|>").copied();
        let im_end_id = token_to_id.get("<|im_end|>").copied();

        Ok(Tokenizer {
            vocab,
            token_to_id,
            merges,
            bos_id,
            eos_id,
            pad_id,
            im_start_id,
            im_end_id,
        })
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Look up a token string directly in the vocabulary.
    /// Useful for special tokens like `<|im_start|>` that exist as
    /// literal vocab entries and shouldn't go through BPE.
    pub fn token_id(&self, name: &str) -> Option<u32> {
        self.token_to_id.get(name).copied()
    }

    /// Encode text that may contain special tokens.
    ///
    /// Splits the input on any occurrence of a token in `special_tokens`,
    /// BPE-encodes the non-special segments, and inserts the correct
    /// token IDs for the special tokens. This prevents BPE from
    /// decomposing special tokens into byte fragments.
    pub fn encode_with_special(&self, text: &str, special_tokens: &[&str]) -> Vec<u32> {
        if special_tokens.is_empty() {
            return self.encode(text);
        }
        let mut result = Vec::new();
        let mut remaining = text;
        while !remaining.is_empty() {
            // Find the earliest occurrence of any special token
            let mut earliest: Option<(usize, &str)> = None;
            for &st in special_tokens {
                if let Some(pos) = remaining.find(st) {
                    if earliest.is_none() || pos < earliest.unwrap().0 {
                        earliest = Some((pos, st));
                    }
                }
            }
            match earliest {
                Some((pos, st)) => {
                    // BPE-encode text before the special token
                    if pos > 0 {
                        result.extend(self.encode(&remaining[..pos]));
                    }
                    // Insert the special token's ID directly
                    if let Some(id) = self.token_id(st) {
                        result.push(id);
                    }
                    remaining = &remaining[pos + st.len()..];
                }
                None => {
                    // No more special tokens — BPE the rest
                    result.extend(self.encode(remaining));
                    break;
                }
            }
        }
        result
    }

    /// Encode text into token IDs using byte-level BPE.
    pub fn encode(&self, text: &str) -> Vec<u32> {
        if text.is_empty() {
            return vec![];
        }

        // Byte-level BPE: convert each byte to its token representation.
        // Qwen/GPT-2 tokenizers map bytes to unicode characters so every
        // byte is representable. The GGUF vocab already uses this encoding.
        //
        // Start with one symbol per byte (as the byte's token string),
        // then iteratively merge the highest-priority pair.
        let mut symbols: Vec<String> = text
            .as_bytes()
            .iter()
            .map(|&b| byte_to_token(b))
            .collect();

        // Iterative BPE merging
        loop {
            if symbols.len() < 2 {
                break;
            }

            // Find the merge pair with lowest priority (= highest rank = should merge first)
            let mut best: Option<(u32, usize)> = None;
            for i in 0..symbols.len() - 1 {
                if let Some(&priority) = self.merges.get(&(symbols[i].clone(), symbols[i + 1].clone())) {
                    if best.is_none() || priority < best.unwrap().0 {
                        best = Some((priority, i));
                    }
                }
            }

            let Some((_, idx)) = best else { break };

            // Merge the pair at idx
            let merged = format!("{}{}", symbols[idx], symbols[idx + 1]);
            symbols[idx] = merged;
            symbols.remove(idx + 1);
        }

        // Look up each merged symbol in the vocab
        symbols
            .iter()
            .map(|s| {
                self.token_to_id
                    .get(s)
                    .copied()
                    .unwrap_or(0) // fallback to token 0 for unknown
            })
            .collect()
    }

    /// Decode token IDs back to text.
    pub fn decode(&self, ids: &[u32]) -> String {
        let mut bytes = Vec::new();
        for &id in ids {
            let token = &self.vocab[id as usize];
            for ch in token.chars() {
                if let Some(b) = token_char_to_byte(ch) {
                    bytes.push(b);
                } else {
                    // Non-byte-mapped character (e.g. special tokens) — emit as UTF-8
                    let mut buf = [0u8; 4];
                    let s = ch.encode_utf8(&mut buf);
                    bytes.extend_from_slice(s.as_bytes());
                }
            }
        }
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Decode a single token ID to its string representation.
    pub fn decode_token(&self, id: u32) -> &str {
        &self.vocab[id as usize]
    }
}

/// GPT-2 / Qwen byte-to-unicode mapping.
///
/// Maps each byte (0-255) to a unicode character that's in the vocab.
/// Printable ASCII/Latin characters map to themselves. Control characters
/// and high bytes get shifted to the range U+0100+.
fn byte_to_token(b: u8) -> String {
    let ch = byte_to_unicode(b);
    String::from(ch)
}

fn byte_to_unicode(b: u8) -> char {
    // GPT-2 byte encoder: printable characters stay as-is,
    // everything else maps to U+0100 + offset.
    //
    // The "printable" ranges in GPT-2's bytes_to_unicode:
    //   '!' ..= '~'  (33..=126)
    //   '¡' ..= '¬'  (161..=172)
    //   '®' ..= 'ÿ'  (174..=255)
    //
    // Everything outside those ranges gets mapped to 256+n where n
    // is the count of bytes seen so far that needed remapping.
    GPT2_BYTE_TO_UNICODE[b as usize]
}

fn token_char_to_byte(ch: char) -> Option<u8> {
    let idx = GPT2_UNICODE_TO_BYTE.iter().position(|&c| c == ch)?;
    Some(idx as u8)
}

// Build the GPT-2 byte↔unicode tables at compile time would be ideal,
// but we can use a lazy static approach. For simplicity, build them as
// const arrays.

const fn build_gpt2_tables() -> ([char; 256], [char; 256]) {
    let mut byte_to_uni = ['\0'; 256];
    let mut uni_to_byte = ['\0'; 256]; // uni_to_byte[byte_value] = unicode_char
    let mut n = 0u32;
    let mut b: u32 = 0;
    while b < 256 {
        let is_printable = (b >= 33 && b <= 126)
            || (b >= 161 && b <= 172)
            || (b >= 174 && b <= 255);
        let ch = if is_printable {
            b
        } else {
            let mapped = 256 + n;
            n += 1;
            mapped
        };
        // Safety: all values are valid unicode code points
        byte_to_uni[b as usize] = unsafe { char::from_u32_unchecked(ch) };
        uni_to_byte[b as usize] = unsafe { char::from_u32_unchecked(ch) };
        b += 1;
    }
    (byte_to_uni, uni_to_byte)
}

const GPT2_TABLES: ([char; 256], [char; 256]) = build_gpt2_tables();
const GPT2_BYTE_TO_UNICODE: [char; 256] = GPT2_TABLES.0;
const GPT2_UNICODE_TO_BYTE: [char; 256] = GPT2_TABLES.1;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn byte_unicode_roundtrip() {
        for b in 0..=255u8 {
            let ch = byte_to_unicode(b);
            let back = token_char_to_byte(ch);
            assert_eq!(back, Some(b), "roundtrip failed for byte {b}");
        }
    }

    #[test]
    fn tokenizer_from_gguf() {
        let path = Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }

        let gguf = GgufFile::open(path).unwrap();
        let tok = Tokenizer::from_gguf(&gguf).unwrap();

        println!("vocab size: {}", tok.vocab_size());
        assert_eq!(tok.vocab_size(), 151669);

        // Encode and decode a simple string
        let text = "Hello world";
        let ids = tok.encode(text);
        let decoded = tok.decode(&ids);
        println!("encode({text:?}) = {ids:?}");
        println!("decode back: {decoded:?}");
        assert_eq!(decoded, text);
    }

    #[test]
    fn special_tokens_exist() {
        let path = Path::new("/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf");
        if !path.exists() {
            eprintln!("skipping: model file not found");
            return;
        }

        let gguf = GgufFile::open(path).unwrap();
        let tok = Tokenizer::from_gguf(&gguf).unwrap();

        let im_start = tok.im_start_id.expect("<|im_start|> not in vocab");
        let im_end = tok.im_end_id.expect("<|im_end|> not in vocab");
        println!("<|im_start|> = {im_start}");
        println!("<|im_end|> = {im_end}");

        // Verify encode_with_special handles them correctly
        let text = "<|im_start|>system\nYou are helpful.<|im_end|>";
        let special = &["<|im_start|>", "<|im_end|>"];
        let ids = tok.encode_with_special(text, special);
        assert_eq!(ids[0], im_start);
        assert_eq!(*ids.last().unwrap(), im_end);
        println!("chat template tokens: {ids:?}");
    }
}
