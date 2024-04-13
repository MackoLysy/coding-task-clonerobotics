use std::error::Error;

mod args;
mod errors;
mod logger;
mod serial_port;
mod server;

use args::ArgParser;
use clap::Parser;
use serial_port::SerialPort;
use server::HttpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::enable_logger();
    let config = ArgParser::parse();
    let parser = Box::new(serial_port::parser::Parser::new());
    let mut server = HttpServer::new(config.get_host().to_string(), config.get_port());
    let mut serial_port =
        SerialPort::new(config.get_serial_port().to_string(), config.get_baudrate());
    serial_port.set_parser(parser);
    serial_port.init()?;

    tokio::try_join!(server.start(), serial_port.start())?;
    Ok(())
}
