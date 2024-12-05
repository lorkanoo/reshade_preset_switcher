pub mod time_periods;

use crate::config::preset_rule::rule_condition::condition_data::time_periods::TimePeriods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionData {
    Maps(Vec<u32>),
    BlacklistedMaps(Vec<u32>),
    Time(TimePeriods),
    Chance(f32),
}
