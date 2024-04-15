use std::error::Error;
use std::time::{Duration, SystemTime};

mod args;
mod data_base;
mod device;
mod errors;
mod logger;
mod serial_port;
mod server;

use args::ArgParser;
use clap::Parser;
use data_base::DataBase;
use device::Device;
use serial_port::SerialPort;
use server::HttpServer;
use std::sync::{Arc, Mutex};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::enable_logger();

    let config = ArgParser::parse();
    let server = HttpServer::new(config.get_host().to_string(), config.get_port());

    let mut dbtemp = DataBase::new(config.get_databse_path().to_string())?;
    dbtemp.createTable()?;
    let db = Arc::new(Mutex::new(dbtemp));
    let device = Arc::new(Mutex::new(Device::new()));

    let mut serial_porttemp =
        SerialPort::new(config.get_serial_port().to_string(), config.get_baudrate());
    let serial_port = Arc::new(tokio::sync::Mutex::new(serial_porttemp));
    serial_port
        .clone()
        .lock()
        .await
        .set_readable(device.clone());
    serial_port.clone().lock().await.set_parser();
    serial_port.clone().lock().await.init()?;

    {
        device.clone().lock().unwrap().set_data_base(db.clone());
    }
    let serial_port_clone = serial_port.clone();

    // let device_task = async {
    //     {
    //         device.lock().unwrap().run().await?;
    //     }
    //     Ok::<(), Box<dyn std::error::Error>>(())
    // };
    // let serial_task = async {
    //     {
    //         serial_port.lock().unwrap().start().await?;
    //     }
    //     Ok::<(), Box<dyn std::error::Error>>(())
    // };
    let server_task = tokio::spawn(async move {
        server.start().await;
    });
    let mut result = serial_port_clone.lock().await;
    let serial_task = tokio::spawn(async move {
        result.start().await;
    });
    // Spawn the serial port task
    // let serial_task = tokio::spawn(async move {
    //     let result = serial_port_clone.lock().await;
    //     result.s
    //     // let lock = serial_port.lock().await.start().await;
    // });
    tokio::join!(server_task);
    // tokio::try_join!(server.start(), serial_task)?;
    Ok(())
}
