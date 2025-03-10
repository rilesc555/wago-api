#[derive(Debug)]
pub struct PlcDriver {
    pub properties: Properties,
    pub queue_handle: Option<tokio::task::JoinHandle<()>>,
}
