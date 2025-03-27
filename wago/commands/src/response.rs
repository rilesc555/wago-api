use crate::data_types::types::{LoadCell, SolenoidCommand};
use std::error::Error;

#[derive(Debug)]
pub enum Response {
    LoadCellResponse(LoadCell),
    ToolProbeResponse,
    TempSensorResponse,
    PressureGaugeResponse,
    SetSolenoidResponse(SolenoidCommand),
    WriteError(Box<dyn Error + Send + Sync>),
    ReadError(Box<dyn Error + Send + Sync>),
}

impl Clone for Response {
    fn clone(&self) -> Self {
        match self {
            Self::LoadCellResponse(lc) => Self::LoadCellResponse(lc.clone()),
            Self::ToolProbeResponse => Self::ToolProbeResponse,
            Self::TempSensorResponse => Self::TempSensorResponse,
            Self::PressureGaugeResponse => Self::PressureGaugeResponse,
            Self::SetSolenoidResponse(cmd) => Self::SetSolenoidResponse(cmd.clone()),
            Self::WriteError(e) => Self::WriteError(e.to_string().into()),
            Self::ReadError(e) => Self::ReadError(e.to_string().into()),
        }
    }
}
