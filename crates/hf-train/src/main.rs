use std::path::Path;

use anyhow::Result;
use burn::tensor::{Int, Tensor, TensorData};

type B = burn::backend::Cuda;

use hf_model::training::{AdamConfig, ManualTrainer};

fn main() -> Result<()> {
    let model_path = std::env::args().nth(1)
        .unwrap_or_else(|| "/home/eric/projb/Bonsai-1.7B-gguf/Bonsai-1.7B.gguf".to_string());
    let path = Path::new(&model_path);
    anyhow::ensure!(path.exists(), "model file not found: {}", path.display());

    let device = burn::backend::cuda::CudaDevice::default();

    // Load model
    println!("loading model...");
    let (model, tokenizer, rope) = hf_model::load_model(path, &device)?;
    println!("  {} layers, d_model={}, vocab={}",
        model.config.n_layers, model.config.d_model, model.config.vocab_size);

    // Initialize trainer (dequantizes all weights to CPU shadow copies)
    println!("initializing trainer (dequantizing to CPU shadow weights)...");
    let adam_cfg = AdamConfig {
        lr: 1e-4,
        ..AdamConfig::default()
    };
    let mut trainer = ManualTrainer::new(model, rope, tokenizer, adam_cfg);
    println!("  trainer ready");

    // Overfit test: memorize a single sequence
    let target_text = "The ship's reactor is online. All systems nominal.";
    let tokens = trainer.tokenizer.encode(target_text);
    println!("overfit target: {:?} ({} tokens)", target_text, tokens.len());

    let n_steps = 20;
    for step in 0..n_steps {
        let t0 = std::time::Instant::now();
        let loss = trainer.train_step(&tokens);
        let elapsed = t0.elapsed();
        println!("  step {step:>3}: loss = {loss:.4}  ({:.1}s)", elapsed.as_secs_f64());
    }

    // Verify with inference after training
    println!("generating after training...");
    let prompt = "The ship's";
    let input_ids = trainer.tokenizer.encode(prompt);
    let mut state = hf_model::InferenceState::new(&trainer.model.config, &device);
    let params = hf_model::SampleParams::greedy();
    let fused_rope = trainer.model.create_rope(&device);

    let input_tensor = Tensor::<B, 2, Int>::from_data(
        TensorData::new(
            input_ids.iter().map(|&t| t as i64).collect::<Vec<_>>(),
            [1, input_ids.len()],
        ),
        &device,
    );

    let logits = trainer.model.forward(input_tensor, &fused_rope, &mut state);
    let [_b, seq_len, _v] = logits.dims();
    let last = logits
        .slice([0..1, (seq_len - 1)..seq_len, 0..trainer.model.config.vocab_size])
        .reshape([trainer.model.config.vocab_size]);
    let mut next_token = hf_model::sample_token(last, &params);

    print!("  {prompt}");
    for _ in 0..30 {
        print!("{}", trainer.tokenizer.decode(&[next_token]));
        if trainer.tokenizer.eos_id == Some(next_token) { break; }
        let inp = Tensor::<B, 2, Int>::from_data(
            TensorData::new(vec![next_token as i64], [1, 1]),
            &device,
        );
        let logits = trainer.model.forward(inp, &fused_rope, &mut state);
        let logits = logits.reshape([trainer.model.config.vocab_size]);
        next_token = hf_model::sample_token(logits, &params);
    }
    println!();

    println!("done.");
    Ok(())
}
