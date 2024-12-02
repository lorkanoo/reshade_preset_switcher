use function_name::named;
use log::error;
use rdev::{EventType, Key};
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCombination {
    pub key_code: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl Default for KeyCombination {
    fn default() -> Self {
        Self {
            key_code: "".to_string(),
            ctrl: Default::default(),
            shift: Default::default(),
            alt: Default::default(),
        }
    }
}

#[named]
pub fn trigger_key_combination(key_combination: &KeyCombination) {
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

    if let Ok(key) = key_combination.key_code.parse() {
        keys.push(Key::Unknown(key));
    } else {
        error!(
            "[{}] Could not parse reshade keybind: {}",
            function_name!(),
            key_combination.key_code
        );
        return;
    }

    for key in &keys {
        crate::util::send(&EventType::KeyPress(*key));
    }

    thread::sleep(Duration::from_millis(30));

    for key in &keys {
        crate::util::send(&EventType::KeyRelease(*key));
    }

    thread::sleep(Duration::from_millis(1500));
}
