pub mod condition_data;
pub mod conjunction_type;

use crate::config::preset_rule::rule_condition::condition_data::ConditionData;
use crate::config::preset_rule::rule_condition::conjunction_type::ConjunctionType;
use crate::render::util::ui::UiElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(default)]
    pub name: String,
    pub data: ConditionData,
    #[serde(default = "ConjunctionType::default")]
    pub conjunction_type: ConjunctionType,
}

impl RuleCondition {
    pub fn new(rule_data: ConditionData, conjunction_type: ConjunctionType) -> Self {
        RuleCondition {
            name: "".to_string(),
            data: rule_data,
            conjunction_type,
        }
    }
}
impl UiElement for RuleCondition {
    fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn name(&self) -> &String {
        &self.name
    }
}
