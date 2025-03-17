use super::properties::TargetProperties;
//
// use tokio_modbus::prelude::*;
use std::net::SocketAddr;
use std::sync::mpsc;
use tokio::sync::broadcast;
use tokio_modbus::client::tcp;
use tokio_modbus::client::Context;
use tokio_modbus::Error;
use wago_commands::command::Message;
use wago_commands::response::Response;

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
    pub fn connect(
        &mut self,
    ) -> Result<(mpsc::Sender<Message>, broadcast::Receiver<Response>), Error> {
        let properties_clone = self.properties.clone();

        let 
    }
}
