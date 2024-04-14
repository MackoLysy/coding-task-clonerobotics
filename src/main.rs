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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::enable_logger();

    let config = ArgParser::parse();
    let parser = Box::new(serial_port::parser::Parser::new());
    let server = HttpServer::new(config.get_host().to_string(), config.get_port());
    let device = Arc::new(Mutex::new(Device::new()));
    let db = DataBase::new(config.get_databse_path().to_string())?;

    let mut serial_port =
        SerialPort::new(config.get_serial_port().to_string(), config.get_baudrate());
    serial_port.set_parser(parser);
    serial_port.set_readable(device);
    serial_port.init()?;
    
    tokio::try_join!(server.start(), serial_port.start())?;
    Ok(())
}
