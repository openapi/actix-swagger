mod generator;
mod utils;

use openapiv3::OpenAPI;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use crate::utils::parse_schema;

pub fn generate_from_schema() {
    let schema = parse_schema();


}