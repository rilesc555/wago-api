use crate::data_types::types::{ReadPressureGauge, ReadTempSensors, ReadToolProbe};
use crate::solenoid::SetSolenoidCommand;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WriteCommand {
    SetSolenoid(SetSolenoidCommand),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WriteMessage {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum ReadCommand {
    ReadLoadCell,
    ReadPressureGauge(ReadPressureGauge),
    ReadTempSensors(ReadTempSensors),
    ReadToolProbe(ReadToolProbe),
}
