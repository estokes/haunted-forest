use std::path::PathBuf;

use anyhow::Result;
use burn::tensor::{Int, Tensor, TensorData};
use clap::Parser;
use rand::seq::SliceRandom;

type B = burn::backend::Cuda;

use hf_model::training::{AdamConfig, ManualTrainer};
use hf_model::training::data;

#[derive(Clone, Copy, clap::ValueEnum)]
enum Mode {
    /// Pre-train on plain text (all tokens contribute to loss)
    Pretrain,
    /// Fine-tune on chat conversations (only assistant turns in loss)
    Finetune,
}

#[derive(Parser)]
#[command(name = "hf-train", about = "Train Bonsai 1-bit models")]
struct Args {
    /// Path to GGUF model file
    #[arg(long, default_value = "/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf")]
    model: PathBuf,

    /// Path to training data (JSONL)
    #[arg(long)]
    data: PathBuf,

    /// Training mode
    #[arg(long, value_enum, default_value = "finetune")]
    mode: Mode,

    /// Number of epochs
    #[arg(long, default_value = "2")]
    epochs: usize,

    /// Learning rate
    #[arg(long, default_value = "1e-4")]
    lr: f64,

    /// Max sequence length (conversations longer than this are skipped)
    #[arg(long, default_value = "512")]
    max_seq_len: usize,

    /// Run inference test after training
    #[arg(long)]
    test_prompt: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    anyhow::ensure!(args.model.exists(), "model not found: {}", args.model.display());
    anyhow::ensure!(args.data.exists(), "data not found: {}", args.data.display());

    let device = burn::backend::cuda::CudaDevice::default();

    // Load model
    println!("loading model from {}...", args.model.display());
    let (model, tokenizer, rope) = hf_model::load_model(&args.model, &device)?;
    println!("  {} layers, d_model={}, vocab={}",
        model.config.n_layers, model.config.d_model, model.config.vocab_size);

    // Initialize trainer
    println!("initializing trainer (dequantizing to CPU shadow weights)...");
    let adam_cfg = AdamConfig {
        lr: args.lr as f32,
        ..AdamConfig::default()
    };
    let mut trainer = ManualTrainer::new(model, rope, tokenizer, adam_cfg);
    println!("  trainer ready (lr={})", args.lr);

    let mut rng = rand::rng();

    match args.mode {
        Mode::Pretrain => train_pretrain(&args, &mut trainer, &mut rng)?,
        Mode::Finetune => train_finetune(&args, &mut trainer, &mut rng)?,
    }

    // Optional inference test
    if let Some(prompt) = &args.test_prompt {
        run_inference_test(&trainer, prompt, &device);
    }

    println!("done.");
    Ok(())
}

fn train_pretrain(
    args: &Args,
    trainer: &mut ManualTrainer,
    rng: &mut impl rand::Rng,
) -> Result<()> {
    println!("loading pre-training corpus from {}...", args.data.display());
    let texts = data::load_plain_text(&args.data)?;
    println!("  {} text chunks loaded", texts.len());

    // Tokenize all chunks
    let mut sequences: Vec<Vec<u32>> = texts
        .iter()
        .map(|t| trainer.tokenizer.encode(t))
        .filter(|ids| ids.len() >= 2 && ids.len() <= args.max_seq_len)
        .collect();
    println!("  {} sequences after filtering (max_seq_len={})", sequences.len(), args.max_seq_len);

    for epoch in 0..args.epochs {
        sequences.shuffle(rng);
        let mut epoch_loss = 0.0;
        let mut n_steps = 0;

        for (i, ids) in sequences.iter().enumerate() {
            let t0 = std::time::Instant::now();
            let loss = trainer.train_step(ids);
            let elapsed = t0.elapsed();

            epoch_loss += loss;
            n_steps += 1;

            if (i + 1) % 10 == 0 || i == 0 {
                println!("  epoch {}/{} step {}/{}: loss={:.4} ({:.1}s)",
                    epoch + 1, args.epochs, i + 1, sequences.len(),
                    loss, elapsed.as_secs_f64());
            }
        }

        let avg_loss = if n_steps > 0 { epoch_loss / n_steps as f32 } else { 0.0 };
        println!("  epoch {}/{} complete: avg_loss={:.4}", epoch + 1, args.epochs, avg_loss);
    }

    Ok(())
}

fn train_finetune(
    args: &Args,
    trainer: &mut ManualTrainer,
    rng: &mut impl rand::Rng,
) -> Result<()> {
    println!("loading conversations from {}...", args.data.display());
    let conversations = data::load_conversations(&args.data)?;
    println!("  {} conversations loaded", conversations.len());

    // Tokenize all conversations with masks
    let mut examples: Vec<(Vec<u32>, Vec<bool>)> = conversations
        .iter()
        .map(|conv| conv.to_token_ids_with_mask(&trainer.tokenizer))
        .filter(|(ids, _)| ids.len() >= 2 && ids.len() <= args.max_seq_len)
        .collect();
    println!("  {} examples after filtering (max_seq_len={})", examples.len(), args.max_seq_len);

    let total_tokens: usize = examples.iter().map(|(ids, _)| ids.len()).sum();
    let train_tokens: usize = examples.iter()
        .map(|(_, mask)| mask.iter().filter(|&&m| m).count())
        .sum();
    println!("  {} total tokens, {} trainable tokens ({:.0}%)",
        total_tokens, train_tokens, 100.0 * train_tokens as f64 / total_tokens as f64);

    for epoch in 0..args.epochs {
        examples.shuffle(rng);
        let mut epoch_loss = 0.0;
        let mut n_steps = 0;

        for (i, (ids, mask)) in examples.iter().enumerate() {
            let t0 = std::time::Instant::now();
            let loss = trainer.train_step_masked(ids, mask);
            let elapsed = t0.elapsed();

            epoch_loss += loss;
            n_steps += 1;

            if (i + 1) % 10 == 0 || i == 0 {
                println!("  epoch {}/{} step {}/{}: loss={:.4} ({:.1}s)",
                    epoch + 1, args.epochs, i + 1, examples.len(),
                    loss, elapsed.as_secs_f64());
            }
        }

        let avg_loss = if n_steps > 0 { epoch_loss / n_steps as f32 } else { 0.0 };
        println!("  epoch {}/{} complete: avg_loss={:.4}", epoch + 1, args.epochs, avg_loss);
    }

    Ok(())
}

fn run_inference_test(trainer: &ManualTrainer, prompt: &str, device: &burn::backend::cuda::CudaDevice) {
    println!("\n--- inference test ---");
    let input_ids = trainer.tokenizer.encode(prompt);
    let mut state = hf_model::InferenceState::new(&trainer.model.config, device);
    let params = hf_model::SampleParams::greedy();
    let fused_rope = trainer.model.create_rope(device);

    let input_tensor = Tensor::<B, 2, Int>::from_data(
        TensorData::new(
            input_ids.iter().map(|&t| t as i64).collect::<Vec<_>>(),
            [1, input_ids.len()],
        ),
        device,
    );

    let logits = trainer.model.forward(input_tensor, &fused_rope, &mut state);
    let [_b, seq_len, _v] = logits.dims();
    let last = logits
        .slice([0..1, (seq_len - 1)..seq_len, 0..trainer.model.config.vocab_size])
        .reshape([trainer.model.config.vocab_size]);
    let mut next_token = hf_model::sample_token(last, &params);

    print!("  {prompt}");
    for _ in 0..50 {
        print!("{}", trainer.tokenizer.decode(&[next_token]));
        if trainer.tokenizer.eos_id == Some(next_token) { break; }
        let inp = Tensor::<B, 2, Int>::from_data(
            TensorData::new(vec![next_token as i64], [1, 1]),
            device,
        );
        let logits = trainer.model.forward(inp, &fused_rope, &mut state);
        let logits = logits.reshape([trainer.model.config.vocab_size]);
        next_token = hf_model::sample_token(logits, &params);
    }
    println!();
}
