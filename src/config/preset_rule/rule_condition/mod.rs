pub mod rule_data;

use crate::config::preset_rule::rule_condition::rule_data::ConditionData;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub data: ConditionData,
    #[serde(default = "ConjunctionType::default")]
    pub conjunction_type: ConjunctionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConjunctionType {
    Or,
    And,
}

pub trait Switch<T> {
    fn switch(&mut self);
}

impl Switch<ConjunctionType> for ConjunctionType {
    fn switch(&mut self) {
        *self = match self {
            ConjunctionType::Or => ConjunctionType::And,
            ConjunctionType::And => ConjunctionType::Or,
        };
    }
}

impl Default for ConjunctionType {
    fn default() -> Self {
        ConjunctionType::Or
    }
}

impl fmt::Display for ConjunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            ConjunctionType::Or => "Or",
            ConjunctionType::And => "And",
        };
        write!(f, "{}", str)
    }
}

impl RuleCondition {
    pub fn new(rule_data: ConditionData, conjunction_type: ConjunctionType) -> Self {
        RuleCondition {
            data: rule_data,
            conjunction_type,
        }
    }
}
