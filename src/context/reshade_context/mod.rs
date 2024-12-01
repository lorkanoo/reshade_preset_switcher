pub mod key_combination;

use crate::context::reshade_context::key_combination::KeyCombination;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReshadeContext {
    pub preset_shortcut_paths: Vec<PathBuf>,
    pub active_preset_path: PathBuf,
    pub preset_shortcut_keys: Vec<KeyCombination>,
}

impl ReshadeContext {
    pub fn valid(&self) -> bool {
        !self.preset_shortcut_keys.is_empty()
    }
}
