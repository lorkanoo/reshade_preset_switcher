use serde::{Deserialize, Serialize};

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
