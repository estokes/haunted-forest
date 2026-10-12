//! Evaluate a nozzle spec, a cosmic-ray shelter spec, or an ICF blast
//! shell spec, and print the report as JSON.
//!
//! `hf-nozzle [shelter|shell] [-|FILE]`: with no path the default spec, `-`
//! a spec on stdin, else one from FILE. Missing fields take defaults.

use anyhow::{Context, Result};
use hf_nozzle::{ShellSpec, ShelterSpec, Spec};
use serde::de::DeserializeOwned;
use std::{fs::File, io};

fn read<T: DeserializeOwned + Default>(arg: Option<&str>) -> Result<T> {
    Ok(match arg {
        None => T::default(),
        Some("-") => serde_json::from_reader(io::stdin()).context("spec on stdin")?,
        Some(path) => serde_json::from_reader(File::open(path).context(path.to_owned())?)
            .with_context(|| format!("spec in {path}"))?,
    })
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let json = match args.first().map(String::as_str) {
        Some("shelter") => serde_json::to_string(
            &read::<ShelterSpec>(args.get(1).map(String::as_str))?.evaluate()?,
        )?,
        Some("shell") => serde_json::to_string(
            &read::<ShellSpec>(args.get(1).map(String::as_str))?.evaluate()?,
        )?,
        arg => serde_json::to_string(&read::<Spec>(arg)?.evaluate()?)?,
    };
    println!("{json}");
    Ok(())
}
