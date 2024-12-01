pub mod rule_condition;

use crate::config::preset_rule::rule_condition::condition_data::ConditionData;
use crate::config::preset_rule::rule_condition::conjunction_type::ConjunctionType;
use crate::config::preset_rule::rule_condition::RuleCondition;
use crate::context::reshade_context::ReshadeContext;
use crate::context::time_period::CurrentTimePeriod;
use crate::context::Context;
use crate::util::reshade::switch_to_preset;
use function_name::named;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRule {
    pub rule_name: String,
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

    #[named]
    pub fn evaluate(&self, context: &Context, current_map_id: &u32) -> RuleProcessingResult {
        let validation_result = self.validate();
        if validation_result.is_ok() {
            let mut rule_fulfilled = false;
            let rule_condition_iter = &mut self.conditions.iter().peekable();
            while let Some(rule_condition) = rule_condition_iter.next() {
                let condition_fulfilled = match &rule_condition.data {
                    ConditionData::Maps(maps) => maps.contains(current_map_id),
                    ConditionData::Time(time_periods) => {
                        debug!(
                            "[{}] Current time period: {:?}",
                            function_name!(),
                            context.current_time_period
                        );
                        debug!(
                            "[{}] Rule time periods: {:?}",
                            function_name!(),
                            time_periods
                        );
                        match context.current_time_period {
                            CurrentTimePeriod::Day => time_periods.day,
                            CurrentTimePeriod::Dusk => time_periods.dusk,
                            CurrentTimePeriod::Night => time_periods.night,
                            CurrentTimePeriod::Dawn => time_periods.dawn,
                        }
                    }
                };

                match rule_condition_iter.peek() {
                    None => {
                        rule_fulfilled = condition_fulfilled;
                        break;
                    }
                    Some(next) => match next.conjunction_type {
                        ConjunctionType::Or => {
                            if condition_fulfilled {
                                rule_fulfilled = true;
                                break;
                            }
                        }
                        ConjunctionType::And => {
                            if !condition_fulfilled {
                                rule_fulfilled = false;
                                break;
                            }
                        }
                    },
                }
            }

            if rule_fulfilled {
                RuleProcessingResult {
                    validation_result,
                    activate_rule: Ok(true),
                }
            } else {
                RuleProcessingResult {
                    validation_result,
                    activate_rule: Ok(false),
                }
            }
        } else {
            RuleProcessingResult {
                validation_result,
                activate_rule: Err(()),
            }
        }
    }

    pub fn activate(&self, reshade_context: &ReshadeContext) {
        switch_to_preset(&self.preset_path, reshade_context);
    }
}
