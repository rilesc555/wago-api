use crate::data_types::types::{LoadCell, Solenoid};
use std::error::Error;

#[derive(Debug)]
pub enum Response {
    LoadCellResponse(LoadCell),
    ToolProbeResponse,
    TempSensorResponse,
    PressureGaugeResponse,
    SetSolenoidResponse(Solenoid),
    WriteError(Box<dyn Error>),
}
