use anyhow::{Context, Result};
use sqlc_gen_rust::{generate_code, GenerateRequest};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    let request: GenerateRequest =
        serde_json::from_str(&input).context("Failed to parse GenerateRequest from JSON")?;

    let response = generate_code(request)?;

    let output =
        serde_json::to_string(&response).context("Failed to serialize GenerateResponse to JSON")?;

    println!("{output}");
    Ok(())
}
