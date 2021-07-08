use syn::{LitStr};
use std::path::PathBuf;

struct ValidateSchemaArgs {
    route_name: Option<LitStr>,
    target_dir: PathBuf,
    workspace_root: PathBuf,
}