pub mod generator;
pub mod type_mapping;
pub mod types;

pub use generator::RustGenerator;
pub use types::{File, GenerateRequest, GenerateResponse, PluginOptions};

use anyhow::Result;

pub fn generate_code(request: GenerateRequest) -> Result<GenerateResponse> {
    let generator = RustGenerator::new(request);
    generator.generate()
}
