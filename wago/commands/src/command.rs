use crate::solenoid::SetSolenoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    ReadLoadCell,
    ReadToolProbe,
    ReadTempSensors,
    ReadPressureGauge,
    SetSolenoid(SetSolenoid),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub priority: Priority,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Standard,
    High,
    Immediate,
    Termination,
}
