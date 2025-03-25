use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TargetProperties {
    pub ip: String,
    pub LOAD_CELL_LIM: f64,
    pub LOAD_CELL_SENSIT: f64,
    pub PRESS_MIN: f32,
    pub PRESS_MAX: f32,
    pub MV_RANGE: f64,
    pub EXCIT_VOLTAGE: f64,

    /**** CARD REGISTERS/COILS AND ASSIGNMENTS ****/
    // READ REGISTERS/COILS by card

    // 750-1491 - Analog In
    pub R_REG0: u16, // Load Cell 1 Signal +     // Read Register 0 is on Card 750-1491 and assigned to Load Cell 1, Signal + in
    pub R_REG1: u16, // Load Cell 1 Signal -
    pub R_REG2: u16, // Load Cell 2 Signal +
    pub R_REG3: u16, // Load Cell 2 Signal -

    // 750-468 - Analog In
    pub R_REG4: u16, // Digital Pressure Gauge
    pub R_REG5: u16,
    pub R_REG6: u16,
    pub R_REG7: u16,

    // 750-423 - Digital In
    pub R_COIL0: u16, // Tool Probe Trigger
    pub R_COIL1: u16, //
    pub R_COIL2: u16, //
    pub R_COIL3: u16, //

    // 750-469/003 - Analog In
    pub R_REG8: u16,
    pub R_REG9: u16,
    pub R_REG10: u16,
    pub R_REG11: u16,
    // 750-559 - Analog Out
    pub W_REG0: u16, // electro-pneumatic regulator
    pub W_REG1: u16,
    pub W_REG2: u16,
    pub W_REG3: u16,

    // 750-502 - Digital Out
    pub W_COIL0: u16, // Extrude Solenoid (activating this coil sets the valve to extrude)
    pub W_COIL1: u16, // Refill Solenoid (activating this coil sets the valve to refill)

    pub W_COIL6: u16,
    pub W_COIL7: u16,

    W_REG8: u16,
    W_REG9: u16,
    W_REG10: u16,
}

impl TargetProperties {
    pub fn new(ip: &str) -> Self {
        return Self {
            ip: ip.to_string(),
            W_REG0: todo!(),
            W_REG1: todo!(),
            W_REG2: todo!(),
            W_REG3: todo!(),
            W_COIL0: todo!(),
            W_COIL1: todo!(),
            W_COIL6: todo!(),
            W_COIL7: todo!(),
            W_REG8: todo!(),
            W_REG9: todo!(),
            W_REG10: todo!(),
        };
    }
}

impl Default for TargetProperties {
    fn default() -> Self {
        return Self {
            ip: "10.22.1.201:502".to_string(),
            W_REG0: 0,
            W_REG1: 1,
            W_REG2: 2,
            W_REG3: 3,
            W_REG8: 8,
            W_REG9: 9,
            W_REG10: 10,
            W_COIL0: 0,
            W_COIL1: 1,
            W_COIL6: 6,
            W_COIL7: 7,
        };
    }
}

/*

*/
