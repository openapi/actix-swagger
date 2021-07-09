mod generator;
mod utils;

use openapiv3::OpenAPI;
use std::path::PathBuf;
use once_cell::sync::Lazy;

pub use utils::parse_schema;
use crate::generator::CodeGenerator;

pub fn generate() {
    let schema = parse_schema().unwrap();

    let mut buf = String::new();

    let mut generator = CodeGenerator::new(&mut buf, schema);
    generator.generate();
}