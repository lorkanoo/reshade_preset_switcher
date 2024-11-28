use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRule {
    pub rule_name: String,
    pub maps: Vec<u32>,
    pub preset_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleValidationError {
    NoPresetSelected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleProcessingSuccess {
    PresetActivated,
    PresetNotActivated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleProcessingResult {
    pub validation_result: Result<(), RuleValidationError>,
    pub processing_result: Result<RuleProcessingSuccess, ()>,
}

impl Default for PresetRule {
    fn default() -> Self {
        Self {
            rule_name: "Rule".to_string(),
            maps: Vec::new(),
            preset_path: Default::default(),
        }
    }
}

impl PresetRule {
    pub fn validate(&self) -> Result<(), RuleValidationError> {
        if !self.preset_path.exists() {
            return Err(RuleValidationError::NoPresetSelected);
        }
        Ok(())
    }
}
