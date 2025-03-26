use super::properties::TargetProperties;
use std::cell::Cell;
use std::error::Error;
//
// use tokio_modbus::prelude::*;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc};
use strum::IntoEnumIterator;
use tokio::sync::{broadcast, Mutex};
use tokio_modbus::client::tcp;
use tokio_modbus::client::{Context, Reader, Writer};
use wago_commands::command::{ReadCommand, WriteCommand, WriteMessage};
use wago_commands::data_types::types::{LoadCell, SolenoidCommand};
use wago_commands::response::{self, Response};
use wago_commands::solenoid::SetSolenoidCommand;

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
    ) -> Result<
        (mpsc::Sender<WriteMessage>, broadcast::Receiver<Response>),
        Box<dyn std::error::Error>,
    > {
        let properties_clone = self.properties.clone();

        let socket_addr: SocketAddr = properties_clone.ip.parse()?;
        let conn = tcp::connect(socket_addr).await?;

        let conn = Arc::new(Mutex::new(conn));

        self.properties.tare = self.tare(&conn);

        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        let (queue_tx, queue_rx) = mpsc::channel::<WriteMessage>();

        let port_clone = Arc::clone(&conn);

        self.wago_handle = Some(self.spawn_queue_loop(queue_rx, port_clone, channel_tx.clone()));

        Ok((queue_tx, channel_rx))
    }

    fn spawn_queue_loop(
        &mut self,
        queue_rx: mpsc::Receiver<WriteMessage>,
        port: Arc<tokio::sync::Mutex<Context>>,
        channel_tx: broadcast::Sender<Response>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut port = port.lock().await;

            let response_vec: Vec<Response> = Vec::new();
            for command in ReadCommand::iter() {
                let answer = self.send_read_command(command, &mut *port);
                response_vec.push(answer);
            }

            loop {
                let mut queue = Vec::new();
                while let Ok(msg) = queue_rx.try_recv() {
                    queue.push(msg);
                }

                queue.sort_by(|a, b| b.priority.cmp(&a.priority));

                for message in queue {
                    let response = { self.send_write_command(message.command, &mut *port) };

                    let _ = channel_tx.send(response);
                }

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        })
    }

    async fn send_write_command(&mut self, command: WriteCommand, port: &mut Context) -> Response {
        match command {
            WriteCommand::SetSolenoid(set_solenoid) => {
                let command_response = match self
                    .solenoid_write(port, set_solenoid.solenoid_command)
                    .await?
                {
                    Ok(response) => response,
                    Err(e) => Response::WriteError(e),
                };
            }
        }
    }

    async fn send_read_command(
        &self,
        command: ReadCommand,
        port: &mut Context,
    ) -> Result<Response, Box<dyn Error>> {
        match command {
            ReadCommand::ReadLoadCell => {
                let read_response = match self.load_cell_read(port).await? {
                    Ok(response) => response,
                    Err(e) => Response::ReadError(e),
                };
            }
        }
    }

    async fn load_cell_read(&self, port: &mut Context) -> Result<Response, Box<dyn Error>> {
        let (sig1_p, sig1_n, sig2_p, sig2_n) = {
            // call reading function, store result
            (
                port.read_input_registers(R_REG0, 1).await??[0] as i16, // sig1_p
                port.read_input_registers(R_REG1, 1).await??[0] as i16, // sig1_m
                port.read_input_registers(R_REG2, 1).await??[0] as i16, // sig2_p
                port.read_input_registers(R_REG3, 1).await??[0] as i16, // sig2_m
            )
        };

        let load1: f64 = self.analog_lbs(sig1_p, sig1_n, self.properties.tare);
        let load2: f64 = self.analog_lbs(sig2_p, sig2_n, self.properties.tare);

        let load_cell = LoadCell::new(load1, load2);
        response = Response::LoadCellResponse(LoadCell)
    }

    async fn solenoid_write(
        &mut self,
        port: &mut Context,
        solenoid_command: SolenoidCommand,
    ) -> Result<Response, Box<dyn Error>> {
        match solenoid_command {
            SolenoidCommand::Extrude => {
                //switch to extrude
                let _ = port
                    .write_single_coil(self.properties.w_coil1, false)
                    .await?; // turn refill sol off
                let _ = port
                    .write_single_coil(self.properties.w_coil0, true)
                    .await?; // turn extrude sol on
                response = Response::SetSolenoidResponse(solenoid_command)
            }

            SolenoidCommand::Refill => {
                //switch to refill
                let _ = port.write_single_coil(W_COIL0, false).await?; // turn extrude sol off
                let _ = port.write_single_coil(W_COIL1, true).await?; // turn refill sol on
                response = Response::SetSolenoidResponse(solenoid_command)
            }

            SolenoidCommand::Close => {
                //close valves
                let _ = port.write_single_coil(W_COIL1, false).await?; // turn refill sol off
                let _ = port.write_single_coil(W_COIL0, false).await?; // turn extrude sol off
                response = Response::SetSolenoidResponse(solenoid_command)
            }
        }
    }

    async fn tare(&self, port: &Arc<Mutex<Context>>) -> Result<Cell<f64>, Box<dyn Error>> {
        let mut tare_readings: Vec<f64> = Vec::new();
        let mut port = port.lock().await;

        // Loop until 100 successful readings.
        while tare_readings.len() < 100 {
            let (sig1_p, sig1_n) = {
                (
                    port.read_input_registers(R_REG0, 1).await??[0] as i16,
                    port.read_input_registers(R_REG1, 1).await??[0] as i16,
                )
            };

            // Convert the raw readings into a load value (in lbs).
            let load = self.analog_lbs(sig1_p, sig1_n, 0.0);
            tare_readings.push(load);

            // add a small delay between each reading.
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        // Calculate the average of the 100 readings.
        let sum: f64 = tare_readings.iter().sum();
        let average = sum / (tare_readings.len() as f64);
        Ok(average)
    }

    fn analog_lbs(&self, sig1_p: i16, sig1_n: i16, tare: Cell<f64>) -> f64 {
        let props: TargetProperties = self.properties.clone();
        let sig_mv: f64 = (sig_p - sig_n) as f64 * (props.mv_range / 25000.0);
        let force_lbs: f64 =
            (sig_mv / props.excit_voltage) * (props.load_cell_lim / props.load_cell_sensit) - tare;
        force_lbs
    }
}
