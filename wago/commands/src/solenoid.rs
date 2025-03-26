use serde::{Deserialize, Serialize};

use crate::data_types::types::SolenoidCommand;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetSolenoidCommand {
    pub solenoid_command: SolenoidCommand,
}

impl SetSolenoidCommand {
    pub fn new(solenoid: SolenoidCommand) -> Self {
        Self {
            solenoid_command: solenoid,
        }
    }
}

impl Default for SetSolenoidCommand {
    fn default() -> Self {
        Self {
            solenoid_command: SolenoidCommand::Close,
        }
    }
}
