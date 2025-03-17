use super::properties::TargetProperties;

#[derive(Debug)]
pub struct WagoDriver {
    pub properties: TargetProperties,
    pub queue_handle: Option<tokio::task::JoinHandle<()>>,
}

impl WagoDriver {
    pub fn new(properties: TargetProperties) -> Self {
        Self {
            properties,
            queue_handle: None,
        }
    }
    pub fn connect(&mut self) {}
}
