pub mod condition_data;
pub mod conjunction_type;

use crate::config::preset_rule::rule_condition::condition_data::ConditionData;
use crate::config::preset_rule::rule_condition::conjunction_type::ConjunctionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub data: ConditionData,
    #[serde(default = "ConjunctionType::default")]
    pub conjunction_type: ConjunctionType,
}

impl RuleCondition {
    pub fn new(rule_data: ConditionData, conjunction_type: ConjunctionType) -> Self {
        RuleCondition {
            data: rule_data,
            conjunction_type,
        }
    }
}
