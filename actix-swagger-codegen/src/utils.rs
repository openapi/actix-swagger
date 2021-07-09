use openapiv3::{Response, PathItem, OpenAPI};
use crate::generator::CodeGenerator;
use convert_case::{Casing, Case};
use std::path::PathBuf;
use once_cell::sync::Lazy;

#[derive(thiserror::Error, Debug)]
pub enum OpenApiError {
    #[error("OpenApi schema not found in crate directory!")]
    NotFound,
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error deserializing yaml schema: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("Error deserializing json schema: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub struct Metadata {
    pub manifest_dir: PathBuf,
}

pub static METADATA: Lazy<Metadata> = Lazy::new(|| {
    use std::env;

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("`CARGO_MANIFEST_DIR` must be set")
        .into();

    Metadata { manifest_dir }
});

pub fn parse_schema() -> Result<OpenAPI, OpenApiError> {
    let yaml_schema = METADATA.manifest_dir.join("openapi.yaml");
    if yaml_schema.exists() {
        return Ok(serde_yaml::from_str(&std::fs::read_to_string(
            yaml_schema,
        )?)?);
    }
    let json_schema = METADATA.manifest_dir.join("openapi.json");
    if json_schema.exists() {
        return Ok(serde_json::from_str(&std::fs::read_to_string(
            json_schema,
        )?)?);
    }

    Err(OpenApiError::NotFound)
}