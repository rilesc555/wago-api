use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TargetProperties {
    pub ip: String,
}

impl TargetProperties {
    pub fn new(ip: &str) -> Self {
        return Self { ip: ip.to_string() };
    }
}

impl Default for TargetProperties {
    fn default() -> Self {
        return Self {
            ip: "10.22.1.201:502".to_string(),
        };
    }
}

/*

*/
