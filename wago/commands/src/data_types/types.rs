use std::f64;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SolenoidCommand {
    Extrude,
    Refill,
    Close,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoadCell {
    pub load1: f64,
    pub load2: f64,
}

impl LoadCell {
    pub fn new(load1: f64, load2: f64) -> Self {
        Self { load1, load2 }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReadLoadCell {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReadToolProbe {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReadTempSensors {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReadPressureGauge {}
