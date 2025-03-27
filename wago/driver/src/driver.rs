use super::properties::TargetProperties;
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
use wago_commands::response::Response;

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
    ) -> Result<(mpsc::Sender<WriteMessage>, broadcast::Receiver<Response>), Box<dyn Error>> {
        let properties_clone = self.properties.clone();

        let socket_addr: SocketAddr = properties_clone.ip.parse()?;
        let conn = tcp::connect(socket_addr).await?;

        let conn = Arc::new(Mutex::new(conn));

        match tare(&conn, &properties_clone).await {
            Ok(tare_value) => {
                self.properties.tare = tare_value;
            }
            Err(e) => return Err(e),
        }
        let properties_clone = self.properties.clone();

        let (channel_tx, channel_rx) = broadcast::channel::<Response>(100);
        let (queue_tx, queue_rx) = mpsc::channel::<WriteMessage>();

        let port_clone = Arc::clone(&conn);

        self.wago_handle = Some(spawn_queue_loop(
            queue_rx,
            port_clone,
            channel_tx.clone(),
            properties_clone,
        ));

        Ok((queue_tx, channel_rx))
    }
}

fn spawn_queue_loop(
    queue_rx: mpsc::Receiver<WriteMessage>,
    port: Arc<Mutex<Context>>,
    channel_tx: broadcast::Sender<Response>,
    props: TargetProperties,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        for command in ReadCommand::iter() {
            let response = send_read_command(command, &port, &props).await;
            let _ = channel_tx.send(response);
        }

        loop {
            let mut queue = Vec::new();
            while let Ok(msg) = queue_rx.try_recv() {
                queue.push(msg);
            }

            queue.sort_by(|a, b| b.priority.cmp(&a.priority));

            for message in queue {
                let response = { send_write_command(message.command, &port, &props).await };

                let _ = channel_tx.send(response);
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    })
}

async fn send_write_command(
    command: WriteCommand,
    port: &Arc<Mutex<Context>>,
    props: &TargetProperties,
) -> Response {
    match command {
        WriteCommand::SetSolenoid(set_solenoid) => {
            match solenoid_write(port, set_solenoid.solenoid_command, props).await {
                Ok(response) => response,
                Err(e) => Response::WriteError(e),
            }
        }
        _ => todo!(),
    }
}

async fn send_read_command(
    command: ReadCommand,
    port: &Arc<Mutex<Context>>,
    props: &TargetProperties,
) -> Response {
    let mut context = port.lock().await;
    match command {
        ReadCommand::ReadLoadCell => match context.read_input_registers(props.r_reg0, 4).await {
            Ok(inner_result) => match inner_result {
                Ok(buff) => {
                    let new_vec: Vec<i16> = buff.into_iter().map(|value| value as i16).collect();
                    match &new_vec[..] {
                        &[sig1_p, sig1_n, sig2_p, sig2_n] => {
                            let load1: f64 = analog_lbs(sig1_p, sig1_n, props.tare, props);
                            let load2: f64 = analog_lbs(sig2_p, sig2_n, props.tare, props);

                            let response = Response::LoadCellResponse(LoadCell::new(load1, load2));
                            response
                        }

                        _ => {
                            let error_message = format!(
                                            "Load Cell Read Error: Expected 4 registers for load cells, but received {}",
                                            new_vec.len()
                                        );
                            let error: Box<dyn Error + Sync + Send> = error_message.into();
                            let response = Response::ReadError(error);
                            response
                        }
                    }
                }
                Err(e) => {
                    let response = Response::ReadError(Box::new(e));
                    response
                }
            },
            Err(e) => {
                let response = Response::ReadError(Box::new(e));
                response
            }
        },
        _ => todo!(),
    }
}

//entry function
pub async fn solenoid_write(
    port: &Arc<Mutex<Context>>,
    solenoid_command: SolenoidCommand,
    props: &TargetProperties,
) -> Result<Response, Box<dyn Error + Sync + Send>> {
    let mut context = port.lock().await;

    match solenoid_command {
        SolenoidCommand::Extrude => {
            //switch to extrude
            let _ = context.write_single_coil(props.w_coil1, false).await?; // turn refill sol off
            let _ = context.write_single_coil(props.w_coil0, true).await?; // turn extrude sol on
            let response = Response::SetSolenoidResponse(solenoid_command);
            Ok(response)
        }

        SolenoidCommand::Refill => {
            //switch to refill
            let _ = context.write_single_coil(props.w_coil0, false).await?; // turn extrude sol off
            let _ = context.write_single_coil(props.w_coil1, true).await?; // turn refill sol on
            let response = Response::SetSolenoidResponse(solenoid_command);
            Ok(response)
        }

        SolenoidCommand::Close => {
            //close valves
            let _ = context.write_single_coil(props.w_coil1, false).await?; // turn refill sol off
            let _ = context.write_single_coil(props.w_coil0, false).await?; // turn extrude sol off
            let response = Response::SetSolenoidResponse(solenoid_command);
            Ok(response)
        }
    }
}

async fn tare(port: &Arc<Mutex<Context>>, props: &TargetProperties) -> Result<f64, Box<dyn Error>> {
    let mut tare_readings: Vec<f64> = Vec::new();

    let mut context = port.lock().await;
    // Loop until 100 successful readings.
    while tare_readings.len() < 100 {
        let (sig1_p, sig1_n) = {
            (
                context.read_input_registers(props.r_reg0, 1).await??[0] as i16,
                context.read_input_registers(props.r_reg1, 1).await??[0] as i16,
            )
        };

        // Convert the raw readings into a load value (in lbs).
        let load: f64 = analog_lbs(sig1_p, sig1_n, 0.0, props);
        tare_readings.push(load);

        // add a small delay between each reading.
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    // Calculate the average of the 100 readings.
    let sum: f64 = tare_readings.iter().sum();
    let average = sum / (tare_readings.len() as f64);
    Ok(average)
}

fn analog_lbs(sig_p: i16, sig_n: i16, tare: f64, props: &TargetProperties) -> f64 {
    let sig_mv = (sig_p - sig_n) as f64 * (props.mv_range / 25000.0);
    let force_lbs =
        (sig_mv / props.excit_voltage) * (props.load_cell_lim / props.load_cell_sensit) - tare;
    force_lbs
}
