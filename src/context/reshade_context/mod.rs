pub mod key_combination;

use crate::context::reshade_context::key_combination::KeyCombination;
use function_name::named;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReshadeContext {
    pub preset_shortcut_paths: Vec<PathBuf>,
    pub active_preset_path: PathBuf,
    pub preset_shortcut_keys: Vec<KeyCombination>,
    pub verify_activation: Option<(PathBuf, usize)>,
}

impl ReshadeContext {
    pub fn valid(&self) -> bool {
        !self.preset_shortcut_keys.is_empty()
    }

    #[named]
    pub fn should_retry_activation(&mut self) -> bool {
        if let Some((preset_path, mut retries)) = self.verify_activation.as_ref() {
            if self.active_preset_path == *preset_path {
                self.verify_activation = None;
            } else if retries > 0 {
                retries -= 1;
                info!(
                    "[{}] Retrying activation of preset [{}], retries left: [{}]",
                    function_name!(),
                    preset_path.display(),
                    retries
                );
                self.verify_activation = Some((preset_path.clone(), retries));
                return true;
            }
        }

        self.verify_activation = None;
        false
    }
}
