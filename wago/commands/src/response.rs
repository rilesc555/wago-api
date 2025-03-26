use crate::data_types::types::{LoadCell, SolenoidCommand};
use std::error::Error;

#[derive(Debug)]
pub enum Response {
    LoadCellResponse(LoadCell),
    ToolProbeResponse,
    TempSensorResponse,
    PressureGaugeResponse,
    SetSolenoidResponse(SolenoidCommand),
    WriteError(Box<dyn Error>),
    ReadError(Box<dyn Error>),
}
