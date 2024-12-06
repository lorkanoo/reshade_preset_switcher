pub mod rule_condition;

use crate::config::preset_rule::rule_condition::condition_data::ConditionData;
use crate::config::preset_rule::rule_condition::conjunction_type::ConjunctionType;
use crate::config::preset_rule::rule_condition::RuleCondition;
use crate::context::reshade_context::ReshadeContext;
use crate::context::time_period::CurrentTimePeriod;
use crate::context::Context;
use crate::render::util::ui::UiElement;
use crate::util::reshade::switch_to_preset;
use function_name::named;
use log::debug;
use rand::Rng;
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

impl UiElement for PresetRule {
    fn rename(&mut self, new_name: String) {
        self.rule_name = new_name;
    }

    fn name(&self) -> &String {
        &self.rule_name
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
            let mut inside_failed_and_chain = false;
            let rule_condition_iter = &mut self.conditions.iter().peekable();
            while let Some(rule_condition) = rule_condition_iter.next() {
                let current_condition_fullfiled = {
                    if inside_failed_and_chain {
                        false
                    } else {
                        match &rule_condition.data {
                            ConditionData::Maps(maps) => maps.contains(current_map_id),
                            ConditionData::BlacklistedMaps(maps) => !maps.contains(current_map_id),
                            ConditionData::Time(time_periods) => {
                                match context.current_time_period {
                                    CurrentTimePeriod::Day => time_periods.day,
                                    CurrentTimePeriod::Dusk => time_periods.dusk,
                                    CurrentTimePeriod::Night => time_periods.night,
                                    CurrentTimePeriod::Dawn => time_periods.dawn,
                                }
                            }
                            ConditionData::Chance(chance) => {
                                let mut gen = rand::thread_rng();
                                let roll = gen.gen_range(0.0..=1.0);
                                roll <= *chance
                            }
                        }
                    }
                };

                match rule_condition_iter.peek() {
                    None => {
                        //never true if inside_failed_and_chain
                        rule_fulfilled = current_condition_fullfiled;
                        break;
                    }
                    Some(next) => match next.conjunction_type {
                        ConjunctionType::Or => {
                            //fresh card - ignore all previous failures
                            inside_failed_and_chain = false;
                            if current_condition_fullfiled {
                                rule_fulfilled = true;
                                debug!("[{}] Success because of 'or' part", function_name!());
                                break;
                            }
                        }
                        ConjunctionType::And => {
                            //unfulfilled 'and' starts failed_and_chain until 'or' is encountered
                            if !current_condition_fullfiled || inside_failed_and_chain {
                                rule_fulfilled = false;
                                inside_failed_and_chain = true;
                                debug!("[{}] Failure due to 'and' part", function_name!());
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
