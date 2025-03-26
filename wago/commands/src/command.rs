use crate::data_types::types::{ReadLoadCell, ReadPressureGauge, ReadTempSensors, ReadToolProbe};
use crate::solenoid::SetSolenoid;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WriteCommand {
    SetSolenoid(SetSolenoid),
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
    ReadLoadCell(ReadLoadCell),
    ReadPressureGauge(ReadPressureGauge),
    ReadTempSensors(ReadTempSensors),
    ReadToolProbe(ReadToolProbe),
}
