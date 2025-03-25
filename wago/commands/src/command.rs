use crate::solenoid::SetSolenoid;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumIter)]
pub enum ReadCommand {
    ReadLoadCell,
    ReadToolProbe,
    ReadTempSensors,
    ReadPressureGauge,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WriteCommand {
    SetSolenoid(SetSolenoid),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub priority: Priority,
    pub command: WriteCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Standard,
    High,
    Immediate,
    Termination,
}
