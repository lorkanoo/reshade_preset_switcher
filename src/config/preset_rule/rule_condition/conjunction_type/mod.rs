use crate::config::SwitchValue;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConjunctionType {
    #[default]
    Or,
    And,
}

impl SwitchValue<ConjunctionType> for ConjunctionType {
    fn switch(&mut self) {
        *self = match self {
            ConjunctionType::Or => ConjunctionType::And,
            ConjunctionType::And => ConjunctionType::Or,
        };
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
