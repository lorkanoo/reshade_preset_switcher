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

    pub fn as_reshade_shortcut_configuration(&self) -> String {
        let preset_shortcut_paths = format!(
            "PresetShortcutPaths={}",
            self.preset_shortcut_paths
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let mut preset_shortcut_keys = String::new();
        preset_shortcut_keys.push_str("PresetShortcutKeys=");

        let iter = &mut self.preset_shortcut_keys.iter().peekable();
        while let Some(key_combination) = iter.next() {
            preset_shortcut_keys.push_str(key_combination.key_code.as_str());
            preset_shortcut_keys.push(',');
            preset_shortcut_keys.push_str(bool_to_string(key_combination.ctrl).as_str());
            preset_shortcut_keys.push(',');
            preset_shortcut_keys.push_str(bool_to_string(key_combination.shift).as_str());
            preset_shortcut_keys.push(',');
            preset_shortcut_keys.push_str(bool_to_string(key_combination.alt).as_str());

            if iter.peek().is_some() {
                preset_shortcut_keys.push(',');
            }
        }
        format!("{}\n{}", preset_shortcut_keys, preset_shortcut_paths)
    }
}

fn bool_to_string(b: bool) -> String {
    if b {
        "1".to_string()
    } else {
        "0".to_string()
    }
}
