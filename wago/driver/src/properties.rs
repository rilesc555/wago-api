use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TargetProperties {
    pub ip: String,
    pub load_cell_lim: f64,
    pub load_cell_sensit: f64,
    pub press_min: f32,
    pub press_max: f32,
    pub mv_range: f64,
    pub excit_voltage: f64,
    pub tare: f64,

    /**** CARD REGISTERS/COILS AND ASSIGNMENTS ****/
    // READ REGISTERS/COILS by card

    // 750-1491 - Analog In
    pub r_reg0: u16, // Load Cell 1 Signal +     // Read Register 0 is on Card 750-1491 and assigned to Load Cell 1, Signal + in
    pub r_reg1: u16, // Load Cell 1 Signal -
    pub r_reg2: u16, // Load Cell 2 Signal +
    pub r_reg3: u16, // Load Cell 2 Signal -

    // 750-468 - Analog In
    pub r_reg4: u16, // Digital Pressure Gauge
    pub r_reg5: u16,
    pub r_reg6: u16,
    pub r_reg7: u16,

    // 750-423 - Digital In
    pub r_coil0: u16, // Tool Probe Trigger
    pub r_coil1: u16, //
    pub r_coil2: u16, //
    pub r_coil3: u16, //

    // 750-469/003 - Analog In
    pub r_reg8: u16,
    pub r_reg9: u16,
    pub r_reg10: u16,
    pub r_reg11: u16,
    // 750-559 - Analog Out
    pub w_reg0: u16, // electro-pneumatic regulator
    pub w_reg1: u16,
    pub w_reg2: u16,
    pub w_reg3: u16,

    // 750-502 - Digital Out
    pub w_coil0: u16, // Extrude Solenoid (activating this coil sets the valve to extrude)
    pub w_coil1: u16, // Refill Solenoid (activating this coil sets the valve to refill)

    pub w_coil6: u16,
    pub w_coil7: u16,

    pub w_reg8: u16,
    pub w_reg9: u16,
    pub w_reg10: u16,
}

impl TargetProperties {
    pub fn new(ip: &str) -> Self {
        return Self {
            ip: ip.to_string(),
            w_reg0: todo!(),
            w_reg1: todo!(),
            w_reg2: todo!(),
            w_reg3: todo!(),
            w_coil0: todo!(),
            w_coil1: todo!(),
            w_coil6: todo!(),
            w_coil7: todo!(),
            w_reg8: todo!(),
            w_reg9: todo!(),
            w_reg10: todo!(),
            load_cell_lim: todo!(),
            load_cell_sensit: todo!(),
            press_min: todo!(),
            press_max: todo!(),
            mv_range: todo!(),
            excit_voltage: todo!(),
            tare: todo!(),
            r_reg0: todo!(),
            r_reg1: todo!(),
            r_reg2: todo!(),
            r_reg3: todo!(),
            r_reg4: todo!(),
            r_reg5: todo!(),
            r_reg6: todo!(),
            r_reg7: todo!(),
            r_coil0: todo!(),
            r_coil1: todo!(),
            r_coil2: todo!(),
            r_coil3: todo!(),
            r_reg8: todo!(),
            r_reg9: todo!(),
            r_reg10: todo!(),
            r_reg11: todo!(),
        };
    }
}

impl Default for TargetProperties {
    fn default() -> Self {
        return Self {
            ip: "10.22.1.201:502".to_string(),
            w_reg0: 0,
            w_reg1: 1,
            w_reg2: 2,
            w_reg3: 3,
            w_reg8: 8,
            w_reg9: 9,
            w_reg10: 10,
            w_coil0: 0,
            w_coil1: 1,
            w_coil6: 6,
            w_coil7: 7,
            load_cell_lim: 500.0,
            load_cell_sensit: 3.0,
            press_min: 0.0,
            press_max: 90.0,
            mv_range: 15.0,
            excit_voltage: 5.0,
            tare: 0.0,
            r_reg0: 0,
            r_reg1: 1,
            r_reg2: 2,
            r_reg3: 3,
            r_reg4: 4,
            r_reg5: 5,
            r_reg6: 6,
            r_reg7: 7,
            r_coil0: 0,
            r_coil1: 1,
            r_coil2: 2,
            r_coil3: 3,
            r_reg8: 8,
            r_reg9: 9,
            r_reg10: 10,
            r_reg11: 11,
        };
    }
}

/*

*/
