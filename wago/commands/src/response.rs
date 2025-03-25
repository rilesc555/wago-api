use crate::data_types::types::Solenoid;

#[derive(Debug, Clone)]
pub enum ReadResponse {
    LoadCellResponse,
    ToolProbeResponse,
    TempSensorResponse,
    PressureGaugeResponse,
}

pub enum WriteResponse {}

pub enum Response {
    SetSolenoidResponse(Solenoid),
    WriteError(error),
}
