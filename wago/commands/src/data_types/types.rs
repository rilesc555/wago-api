use serde::{Deserialize, Serialize};

pub enum Solenoid {
    Extrude,
    Refill,
    Close,
}
