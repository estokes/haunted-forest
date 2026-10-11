//! Evaluate a nozzle spec and print the report as JSON.
//!
//! `hf-nozzle` runs the default spec, `hf-nozzle -` reads a spec from stdin,
//! `hf-nozzle FILE` reads one from FILE. Missing fields take defaults.

use anyhow::{Context, Result};
use hf_nozzle::Spec;
use std::{fs::File, io};

fn main() -> Result<()> {
    let spec: Spec = match std::env::args().nth(1).as_deref() {
        None => Spec::default(),
        Some("-") => serde_json::from_reader(io::stdin()).context("spec on stdin")?,
        Some(path) => serde_json::from_reader(File::open(path).context(path.to_owned())?)
            .with_context(|| format!("spec in {path}"))?,
    };
    println!("{}", serde_json::to_string(&spec.evaluate()?)?);
    Ok(())
}
