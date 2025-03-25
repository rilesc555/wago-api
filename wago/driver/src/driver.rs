use super::properties::TargetProperties;
//
// use tokio_modbus::prelude::*;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc};
use strum::IntoEnumIterator;
use tokio::sync::{broadcast, Mutex};
use tokio_modbus::client::Context;
use tokio_modbus::client::{tcp, Writer};
use wago_commands::command::{Message, ReadCommand, WriteCommand};
use wago_commands::data_types::types::Solenoid;
use wago_commands::response::{self, ReadResponse, Response, WriteResponse};
use wago_commands::solenoid::SetSolenoid;

#[derive(Debug)]
pub struct WagoDriver {
    pub properties: TargetProperties,
    pub wago_handle: Option<tokio::task::JoinHandle<()>>,
}

impl WagoDriver {
    pub fn new(properties: TargetProperties) -> Self {
        Self {
            properties,
            wago_handle: None,
        }
    }
    pub async fn connect(
        &mut self,
    ) -> Result<(mpsc::Sender<Message>, broadcast::Receiver<Response>), Box<dyn std::error::Error>>
    {
        let properties_clone = self.properties.clone();

        let socket_addr: SocketAddr = properties_clone.ip.parse()?;
        let conn = tcp::connect(socket_addr).await?;

        let conn = Arc::new(Mutex::new(conn));

        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        let (queue_tx, queue_rx) = mpsc::channel::<Message>();

        let port_clone = Arc::clone(&conn);

        self.wago_handle = Some(spawn_queue_loop(queue_rx, port_clone, channel_tx.clone()));

        Ok((queue_tx, channel_rx))
    }
}

fn spawn_queue_loop(
    queue_rx: mpsc::Receiver<Message>,
    port: Arc<tokio::sync::Mutex<Context>>,
    channel_tx: broadcast::Sender<Response>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut port = port.lock().await;
        // right here we need to read from all the read registers. For each data_type, response = get_data(data_type) and channel_tx.send(response).
        // This means that response has to either be a datatype or an okay
        //
        // pressure = send_read_command(GetPressure)
        //
        //
        // for Command in ReadCommand {
        //     response: ReadResponse = send_read_command(Command)
        //     for
        // }
        let data_vec: Vec<ReadResponse> = Vec::new();
        for command in ReadCommand::iter() {
            response = send_read_command(command, &mut port);
        }

        loop {
            let mut queue = Vec::new();
            while let Ok(msg) = queue_rx.try_recv() {
                queue.push(msg);
            }

            queue.sort_by(|a, b| b.priority.cmp(&a.priority));

            for message in queue {
                let response = { send_write_command(message.command, &mut *port) };

                let _ = channel_tx.send(response);
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
}

async fn send_write_command(command: WriteCommand, port: &mut Context) -> Response {
    match command {
        WriteCommand::SetSolenoid(set_solenoid) => {
            let command_response = match solenoid_write(port, set_solenoid.solenoid).await? {
                Ok(response) => response,
                Err(e) => Response::WriteError(e),
            };
        }
    }
}

fn send_read_command(command: ReadCommand, port: &mut Context) -> ReadResponse {
    match command {
        ReadCommand::ReadLoadCell => ReadResponse::LoadCellResponse,
        ReadCommand::ReadToolProbe => ReadResponse::ToolProbeResponse,
        ReadCommand::ReadTempSensors => ReadResponse::TempSensorResponse,
        ReadCommand::ReadPressureGauge => ReadResponse::PressureGaugeResponse,
    }
}

//entry function
pub async fn solenoid_write(
    port: &mut Context,
    solenoid_command: Solenoid,
) -> Result<Response, Box<dyn Error>> {
    loop {
        match solenoid_command {
            Solenoid::Extrude => {
                //switch to extrude
                let mut locked_plc = plc.lock().await;
                let _ = locked_plc.write_single_coil(W_COIL1, false).await?; // turn refill sol off
                let _ = locked_plc.write_single_coil(W_COIL0, true).await?; // turn extrude sol on
                response = Response::SetSolenoidResponse(solenoid_command)
            }

            Solenoid::Refill => {
                //switch to refill
                let mut locked_plc = plc.lock().await;
                let _ = locked_plc.write_single_coil(W_COIL0, false).await?; // turn extrude sol off
                let _ = locked_plc.write_single_coil(W_COIL1, true).await?; // turn refill sol on
                response = Response::SetSolenoidResponse(solenoid_command)
            }

            Solenoid::Close => {
                //close valves
                let mut locked_plc = plc.lock().await;
                let _ = locked_plc.write_single_coil(W_COIL1, false).await?; // turn refill sol off
                let _ = locked_plc.write_single_coil(W_COIL0, false).await?; // turn extrude sol off
                response = Response::SetSolenoidResponse(solenoid_command)
            }
        }
    }
}
