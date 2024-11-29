pub mod rule_condition;

use crate::config::preset_rule::rule_condition::RuleCondition;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRule {
    pub rule_name: String,
    //todo: to be removed
    #[serde(skip_serializing, default)]
    pub maps: Vec<u32>,
    pub preset_path: PathBuf,
    #[serde(default)]
    pub conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleValidationError {
    NoPresetSelected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleProcessingResult {
    pub validation_result: Result<(), RuleValidationError>,
    pub activate_rule: Result<bool, ()>,
}

impl Default for PresetRule {
    fn default() -> Self {
        Self {
            rule_name: "Rule".to_string(),
            maps: Vec::new(),
            preset_path: Default::default(),
            conditions: Vec::new(),
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
