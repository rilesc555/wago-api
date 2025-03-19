use super::properties::TargetProperties;
//
// use tokio_modbus::prelude::*;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc};
use tokio::sync::{broadcast, Mutex};
use tokio_modbus::client::tcp;
use tokio_modbus::client::Context;
use wago_commands::command::{Command, Message};
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
    pub async fn connect(
        &mut self,
    ) -> Result<(mpsc::Sender<Message>, broadcast::Receiver<Response>), Box<dyn std::error::Error>>
    {
        let properties_clone = self.properties.clone();

        let socket_addr: SocketAddr = properties_clone.ip.parse()?;
        let port = tcp::connect(socket_addr).await?;

        let port = Arc::new(Mutex::new(port));

        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        let (queue_tx, queue_rx) = mpsc::channel::<Message>();

        let port_clone = Arc::clone(&port);

        self.queue_handle = Some(spawn_queue_loop(queue_rx, port_clone, channel_tx.clone()));

        Ok((queue_tx, channel_rx))
    }
}

fn spawn_queue_loop(
    queue_rx: mpsc::Receiver<Message>,
    port: Arc<tokio::sync::Mutex<Context>>,
    channel_tx: broadcast::Sender<Response>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let mut queue = Vec::new();
            while let Ok(msg) = queue_rx.try_recv() {
                queue.push(msg);
            }

            queue.sort_by(|a, b| b.priority.cmp(&a.priority));

            for message in queue {
                let response = {
                    let mut port = port.lock().await;
                    send_command(message.command, &mut *port)
                };

                let _ = channel_tx.send(response);
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
}

fn send_command(command: Command, port: &mut Context) -> Response {}
