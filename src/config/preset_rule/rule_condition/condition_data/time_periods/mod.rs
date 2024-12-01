use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriods {
    pub day: bool,
    pub dusk: bool,
    pub night: bool,
    pub dawn: bool,
}

impl Default for TimePeriods {
    fn default() -> TimePeriods {
        Self {
            day: true,
            dusk: true,
            night: true,
            dawn: true,
        }
    }
}
