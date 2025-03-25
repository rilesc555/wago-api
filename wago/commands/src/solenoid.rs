use serde::{Deserialize, Serialize};

use crate::data_types::types::Solenoid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetSolenoid {
    pub solenoid: Solenoid,
}

impl SetSolenoid {
    pub fn new(solenoid: Solenoid) -> Self {
        Self { solenoid }
    }
}

impl Default for SetSolenoid {
    fn default() -> Self {
        Self {
            solenoid: Solenoid::Close,
        }
    }
}
