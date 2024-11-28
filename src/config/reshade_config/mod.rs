use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReshadeConfig {
    pub ini_path: PathBuf,
}
