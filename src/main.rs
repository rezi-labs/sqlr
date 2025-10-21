mod types;
mod generator;
mod type_mapping;

use anyhow::{Context, Result};
use std::io::{self, Read};
use types::{GenerateRequest, GenerateResponse};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    let request: GenerateRequest = serde_json::from_str(&input)
        .context("Failed to parse GenerateRequest from JSON")?;

    let response = generate_code(request)?;

    let output = serde_json::to_string(&response)
        .context("Failed to serialize GenerateResponse to JSON")?;

    println!("{}", output);
    Ok(())
}

fn generate_code(request: GenerateRequest) -> Result<GenerateResponse> {
    let generator = generator::RustGenerator::new(request);
    generator.generate()
}