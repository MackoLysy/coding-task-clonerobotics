pub mod parser;
mod serial_port;
use crate::device::Device;
use crate::device::IReadable;
use crate::serial_port::parser::Parser;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

pub struct SerialPort {
    port_name: String,
    baudrate: u32,
    port: Option<Arc<Mutex<serial2_tokio::SerialPort>>>,
    parser: Option<Arc<Mutex<dyn IParser>>>,
    readable: Option<Arc<Mutex<Device>>>,
}

impl SerialPort {
    pub fn set_parser(&mut self) {
        self.parser = Some(Arc::new(Mutex::new(Parser::new())))
    }
    pub fn set_readable(&mut self, val: Arc<Mutex<Device>>) {
        self.readable = Some(val);
    }
}

pub trait IWrittable {
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>>;
}
pub trait IParser: Send {
    fn parse(&mut self, data: u8) -> Option<String>;
    fn parse_vec(&mut self, data: Vec<u8>) -> Option<String>;
}
