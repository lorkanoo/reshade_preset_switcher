use crate::addon::Addon;
use crate::context::reshade_context::key_combination::{trigger_key_combination, KeyCombination};
use crate::context::reshade_context::ReshadeContext;
use crate::util::error;
use crate::util::true_if_1;
use bimap::BiMap;
use function_name::named;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub fn load_reshade_context(reshade_ini_path: &PathBuf) {
    if let Ok(content) = fs::read_to_string(reshade_ini_path) {
        load_active_preset_path(&content);
        load_presets(&content);
        Addon::lock().config.reshade.ini_path = reshade_ini_path.clone();
    }
}

fn load_active_preset_path(content: &str) {
    let re = Regex::new(r"(?m)^PresetPath=(.*)$").unwrap();
    if let Some(captures) = re.captures(content) {
        let preset_path = PathBuf::from(&captures[1].trim());
        Addon::lock().context.reshade.active_preset_path = preset_path;
    }
}

fn load_presets(content: &str) {
    let mut invalid_reshade_preset_configuration = false;
    let mut preset_shortcuts: BiMap<KeyCombination, PathBuf> = BiMap::new();

    let paths_re = Regex::new(r"(?m)^PresetShortcutPaths=(.*)$").unwrap();
    let keys_re = Regex::new(r"(?m)^PresetShortcutKeys=(.*)$").unwrap();
    if let Some(paths_captures) = paths_re.captures(content) {
        if let Some(keys_captures) = keys_re.captures(content) {
            let paths: Vec<PathBuf> = paths_captures[1]
                .trim()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect();
            let keys: Vec<String> = keys_captures[1]
                .trim()
                .split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();

            for (i, chunk) in keys.chunks(4).enumerate() {
                if let Some(path) = paths.get(i) {
                    if path.exists() {
                        let key_code = chunk.first().unwrap().as_str().to_string();
                        let mut key_combination = KeyCombination {
                            key_code,
                            ctrl: false,
                            shift: false,
                            alt: false,
                        };
                        key_combination.ctrl = chunk.get(1).map(true_if_1()).unwrap_or(false);
                        key_combination.shift = chunk.get(2).map(true_if_1()).unwrap_or(false);
                        key_combination.alt = chunk.get(3).map(true_if_1()).unwrap_or(false);
                        preset_shortcuts.insert(key_combination, path.clone());
                    } else {
                        invalid_reshade_preset_configuration = true;
                    }
                }
            }
        }
        Addon::lock().context.reshade.preset_shortcuts = preset_shortcuts;
    }
    Addon::lock()
        .context
        .ui
        .invalid_reshade_preset_configuration = invalid_reshade_preset_configuration;
}

#[named]
pub fn switch_to_preset(preset_path: &PathBuf, context: &ReshadeContext) {
    if let Some(key_combination) = context.preset_shortcuts.get_by_right(preset_path) {
        trigger_key_combination(key_combination);
    } else {
        error!(
            "[{}] Could not find key combination for preset",
            function_name!()
        );
    }
}
