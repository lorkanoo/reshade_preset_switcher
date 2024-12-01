use crate::addon::Addon;
use crate::context::reshade_context::key_combination::{trigger_key_combination, KeyCombination};
use crate::context::reshade_context::ReshadeContext;
use crate::util::true_if_1;
use function_name::named;
use log::{error, warn};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub fn load_reshade_context(reshade_ini_path: &PathBuf) {
    if let Ok(content) = fs::read_to_string(reshade_ini_path) {
        load_active_preset_path(&content);
        load_preset_shortcut_paths(&content);
        load_keybinds(&content);
    }
}

fn load_active_preset_path(content: &str) {
    let re = Regex::new(r"(?m)^PresetPath=(.*)$").unwrap();
    if let Some(captures) = re.captures(content) {
        let preset_path = PathBuf::from(&captures[1].trim());
        Addon::lock().context.reshade.active_preset_path = preset_path;
    }
}

fn load_preset_shortcut_paths(content: &str) {
    let re = Regex::new(r"(?m)^PresetShortcutPaths=(.*)$").unwrap();
    if let Some(captures) = re.captures(content) {
        Addon::lock().context.reshade.preset_shortcut_paths = captures[1]
            .trim()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect();
    }
}

#[named]
fn load_keybinds(content: &str) {
    let re = Regex::new(r"(?m)^PresetShortcutKeys=(.*)$").unwrap();
    let mut preset_shortcut_keys = vec![];
    if let Some(captures) = re.captures(content) {
        let keys: Vec<String> = captures[1]
            .trim()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        for chunk in keys.chunks(4) {
            let key_code = chunk.first().unwrap().as_str().to_string();
            let mut key_combination = KeyCombination {
                key_code,
                ctrl: false,
                shift: false,
                alt: false,
            };
            key_combination.ctrl = keys.get(1).map(true_if_1()).unwrap_or(false);
            key_combination.shift = keys.get(2).map(true_if_1()).unwrap_or(false);
            key_combination.alt = keys.get(3).map(true_if_1()).unwrap_or(false);

            preset_shortcut_keys.push(key_combination);
        }
        Addon::lock().context.reshade.preset_shortcut_keys = preset_shortcut_keys;
    } else {
        warn!("[{}] Could not find keybinds", function_name!());
    }
}

#[named]
pub fn switch_to_preset(preset_path: &PathBuf, context: &ReshadeContext) {
    if let Some(index) = context
        .preset_shortcut_paths
        .iter()
        .position(|path| path == preset_path)
    {
        if let Some(key_combination) = context.preset_shortcut_keys.get(index) {
            trigger_key_combination(key_combination);
        } else {
            error!(
                "[{}] Could not find key combination for preset",
                function_name!()
            );
        }
    } else {
        error!("[{}] Could not find preset path", function_name!());
    }
}
