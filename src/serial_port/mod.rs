pub mod parser;
mod serial_port;
use crate::device::IReadable;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

pub struct SerialPort {
    port_name: String,
    baudrate: u32,
    port: Option<serial2_tokio::SerialPort>,
    parser: Option<Arc<Mutex<Box<dyn IParser>>>>,
    readable: Option<Arc<Mutex<dyn IReadable>>>,
}

impl SerialPort {

    pub fn set_parser(&mut self, parser: Box<dyn IParser>) {
        self.parser = Some(Arc::new(Mutex::new(parser)))
    }
    pub fn set_readable(&mut self, val: Arc<Mutex<dyn IReadable>>) {
        self.readable = Some(val);
    }
}

pub trait IWrittable {
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>>;
}
pub trait IParser {
    fn parse(&mut self, data: u8) -> Option<String>;
    fn parse_vec(&mut self, data: Vec<u8>) -> Option<String>;
}
