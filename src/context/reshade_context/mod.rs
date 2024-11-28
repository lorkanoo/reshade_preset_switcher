pub mod key_combination;

use crate::context::reshade_context::key_combination::KeyCombination;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReshadeContext {
    pub presets_path: PathBuf,
    pub active_preset_path: PathBuf,
    pub previous_preset_key_combination: Option<KeyCombination>,
    pub next_preset_key_combination: Option<KeyCombination>,
    pub presets: Vec<PathBuf>,
}

impl ReshadeContext {
    pub fn valid(&self) -> bool {
        self.next_preset_key_combination.is_some() && self.previous_preset_key_combination.is_some()
    }
}
