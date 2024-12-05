use function_name::named;
use log::error;
use rdev::{EventType, Key};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::time::Duration;
use std::{fmt, thread};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
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

impl fmt::Display for KeyCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();
        if self.ctrl {
            result.push_str("Ctrl+");
        }
        if self.shift {
            result.push_str("Shift+");
        }
        if self.alt {
            result.push_str("Alt+");
        }
        if let Ok(code) = self.key_code.parse::<u32>() {
            if let Some(ch) = char::from_u32(code) {
                result.push(ch);
            }
        }
        write!(f, "{}", result)
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
