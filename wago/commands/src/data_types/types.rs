use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Solenoid {
    Extrude,
    Refill,
    Close,
}
