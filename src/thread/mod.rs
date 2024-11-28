use crate::addon::Addon;
use crate::config::game_dir;
use crate::config::preset_rule::{PresetRule, RuleProcessingResult, RuleProcessingSuccess};
use crate::context::reshade_context::key_combination::KeyCombination;
use crate::context::reshade_context::ReshadeContext;
use function_name::named;
use log::{debug, error, info, warn};
use rdev::{simulate, EventType, Key};
use regex::Regex;
use rfd::FileDialog;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, thread};

pub fn background_thread() {
    Addon::threads().push(thread::spawn(|| loop {
        if !Addon::lock().context.run_background_thread {
            break;
        }
        clean_finished_threads();
        let mut new_map_id: u32 = 0;
        if !Addon::lock().config.valid() {
            info!("Reshade config not valid, skipping background processing");
        } else {
            let reshade_ini_path = Addon::lock().config.reshade.ini_path.clone();
            load_reshade_config(&reshade_ini_path);
            if !Addon::lock().context.valid() {
                info!("Reshade context not valid, skipping background processing");
            } else {
                load_reshade_presets();
                if Addon::lock().context.map_changed(&mut new_map_id) {
                    on_map_change(new_map_id);
                }
            }
        }

        thread::sleep(Duration::from_millis(1000));
    }));
}

pub fn select_reshade_ini_file_thread() {
    Addon::threads().push(thread::spawn(move || {
        if let Some(file) = FileDialog::new()
            .set_title("Select ReShade.ini location")
            .set_directory(game_dir())
            .add_filter("Configuration file (.ini)", &["ini"])
            .pick_file()
        {
            load_reshade_config(&file);
        }
    }));
}

fn load_reshade_config(reshade_ini_path: &PathBuf) {
    if let Ok(content) = fs::read_to_string(reshade_ini_path) {
        load_preset_config(&content);
        load_keybinds(&content);
    }
    Addon::lock().config.reshade.ini_path = reshade_ini_path.clone();
}

fn load_preset_config(content: &str) {
    let re = Regex::new(r"(?m)^PresetPath=(.*)$").unwrap();
    if let Some(captures) = re.captures(content) {
        let preset_path = PathBuf::from(&captures[1].trim());
        if let Some(parent_path) = preset_path.parent() {
            Addon::lock().context.reshade.presets_path = PathBuf::from(parent_path);
        }
        Addon::lock().context.reshade.active_preset_path = preset_path;
    }
}

fn load_keybinds(content: &str) {
    let previous_preset_regex = Regex::new(r"(?m)^KeyPreviousPreset=(.*)$").unwrap();
    let next_preset_regex = Regex::new(r"(?m)^KeyNextPreset=(.*)$").unwrap();
    Addon::lock()
        .context
        .reshade
        .previous_preset_key_combination = load_keybind(content, previous_preset_regex);
    Addon::lock().context.reshade.next_preset_key_combination =
        load_keybind(content, next_preset_regex);
}

fn load_keybind(content: &str, re: Regex) -> Option<KeyCombination> {
    if let Some(captures) = re.captures(content) {
        let keys: Vec<String> = captures[1].trim().split(',').map(String::from).collect();
        if keys.len() != 4 {
            error!("Could not load keybind");
            None
        } else {
            let mut key_combination = KeyCombination {
                key_code: keys.first().unwrap().as_str().to_string(),
                ctrl: false,
                shift: false,
                alt: false,
            };
            if key_combination.key_code == "0" {
                debug!("Keybind not configured in reshade");
                return None;
            }
            key_combination.ctrl = keys.get(1).map(true_if_1()).unwrap_or(false);
            key_combination.shift = keys.get(2).map(true_if_1()).unwrap_or(false);
            key_combination.alt = keys.get(3).map(true_if_1()).unwrap_or(false);
            Some(key_combination)
        }
    } else {
        warn!("Could not find keybind parameter");
        None
    }
}

fn true_if_1() -> fn(&String) -> bool {
    |value| value == "1"
}

fn on_map_change(new_map_id: u32) {
    let addon = Addon::lock();
    let mut preset_activated: bool = false;
    for preset_rule in addon.config.preset_rules.iter() {
        debug!("processing rule {:?}", preset_rule);
        let result = process_rule(
            preset_rule,
            &addon.context.reshade,
            &new_map_id,
            &addon.context.reshade.presets,
        );
        debug!("rule {:?} processed with result {:?}", preset_rule, result);
        if let Ok(success) = result.processing_result {
            if matches!(success, RuleProcessingSuccess::PresetActivated) {
                info!("preset activated");
                preset_activated = true;
                break;
            }
        }
    }
    if !preset_activated {
        let default_preset_rule = addon.config.preset_rules.last();
        if let Some(default_preset_rule) = default_preset_rule {
            info!("Activating default preset");
            let (current_preset_index, default_preset_index) = get_preset_indexes(
                default_preset_rule,
                &addon.context.reshade,
                &addon.context.reshade.presets,
            );
            switch_to_preset(
                &current_preset_index,
                &default_preset_index,
                &addon.context.reshade,
            );
        }
    }
}

pub fn process_rule(
    preset_rule: &PresetRule,
    reshade_context: &ReshadeContext,
    current_map_id: &u32,
    reshade_presets: &[PathBuf],
) -> RuleProcessingResult {
    let validation_result = preset_rule.validate();
    if validation_result.is_ok() {
        if preset_rule.maps.contains(current_map_id) {
            if reshade_context.active_preset_path == preset_rule.preset_path {
                return RuleProcessingResult {
                    validation_result,
                    processing_result: Ok(RuleProcessingSuccess::PresetActivated),
                };
            }
            let (current_preset_index, new_preset_index) =
                get_preset_indexes(preset_rule, reshade_context, reshade_presets);
            switch_to_preset(&current_preset_index, &new_preset_index, reshade_context);
            return RuleProcessingResult {
                validation_result,
                processing_result: Ok(RuleProcessingSuccess::PresetActivated),
            };
        }
        RuleProcessingResult {
            validation_result,
            processing_result: Ok(RuleProcessingSuccess::PresetNotActivated),
        }
    } else {
        RuleProcessingResult {
            validation_result,
            processing_result: Err(()),
        }
    }
}

fn get_preset_indexes(
    preset_rule: &PresetRule,
    reshade_context: &ReshadeContext,
    loaded_presets: &[PathBuf],
) -> (Option<usize>, Option<usize>) {
    let mut current_preset_index = None;
    let mut new_preset_index = None;
    for (index, preset) in loaded_presets.iter().enumerate() {
        if *preset == reshade_context.active_preset_path {
            current_preset_index = Some(index);
        }
        if *preset == preset_rule.preset_path {
            new_preset_index = Some(index);
        }
    }
    (current_preset_index, new_preset_index)
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => info!("Sent"),
        Err(_) => {
            error!("Could not send {:?}", event_type);
        }
    }
}

fn switch_to_preset(
    current_preset_index: &Option<usize>,
    new_preset_index: &Option<usize>,
    reshade_context: &ReshadeContext,
) {
    debug!(
        "current_preset_index: {:?}, new_preset_index: {:?}",
        current_preset_index, new_preset_index
    );
    if current_preset_index.is_none()
        || new_preset_index.is_none()
        || reshade_context.next_preset_key_combination.is_none()
        || reshade_context.previous_preset_key_combination.is_none()
    {
        error!("Could not switch presets");
        return;
    }
    let mut current_preset = current_preset_index.unwrap();
    let new_preset = new_preset_index.unwrap();

    while current_preset != new_preset {
        if current_preset < new_preset {
            trigger_key_combination(
                reshade_context
                    .next_preset_key_combination
                    .as_ref()
                    .unwrap(),
            );
            current_preset += 1;
        } else {
            trigger_key_combination(
                reshade_context
                    .previous_preset_key_combination
                    .as_ref()
                    .unwrap(),
            );
            current_preset -= 1;
        }
    }
}

fn trigger_key_combination(key_combination: &KeyCombination) {
    let mut keys = vec![];
    if key_combination.ctrl {
        keys.push(Key::ControlLeft);
    }
    if key_combination.shift {
        keys.push(Key::ShiftLeft);
    }
    if key_combination.alt {
        keys.push(Key::Alt);
    }
    keys.push(Key::Unknown(key_combination.key_code.parse().unwrap()));

    for key in &keys {
        send(&EventType::KeyPress(*key));
    }

    thread::sleep(Duration::from_millis(20));

    for key in &keys {
        send(&EventType::KeyRelease(*key));
    }
}

pub fn load_reshade_presets() {
    let mut addon = Addon::lock();
    let mut presets = vec![];
    match fs::read_dir(&addon.context.reshade.presets_path) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let preset = entry.path();
                        if preset.is_file() {
                            presets.push(preset);
                        }
                    }
                    Err(e) => error!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => error!("Error reading directory: {}", e),
    }
    addon.context.reshade.presets = presets
}

#[named]
fn clean_finished_threads() {
    Addon::threads().retain(|handle| {
        if handle.is_finished() {
            debug!("[{}] removed finished thread", function_name!());
            false
        } else {
            debug!("[{}] thread in progress..", function_name!());
            true
        }
    });
}