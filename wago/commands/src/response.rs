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

impl From<Response> for String {
    // Use From<Response> which automatically provides Into<String>
    fn from(response: Response) -> String {
        match response {
            Response::LoadCellResponse(lc) => {
                // Assuming LoadCell has a meaningful Debug or Display implementation
                format!("Load Cell Response: {:?}", lc)
            }
            Response::ToolProbeResponse => "Tool Probe Response".to_string(),
            Response::TempSensorResponse => "Temperature Sensor Response".to_string(),
            Response::PressureGaugeResponse => "Pressure Gauge Response".to_string(),
            Response::SetSolenoidResponse(cmd) => {
                // Assuming SolenoidCommand has a meaningful Debug or Display implementation
                format!("Set Solenoid Response: {:?}", cmd)
            }
            Response::WriteError(e) => {
                // Use the error's Display implementation
                format!("Write Error: {}", e)
            }
            Response::ReadError(e) => {
                // Use the error's Display implementation
                format!("Read Error: {}", e)
            }
        }
    }
}
