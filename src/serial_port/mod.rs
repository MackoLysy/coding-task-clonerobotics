pub mod parser;
mod serial_port;
use std::error::Error;

pub trait IWritable {
    async fn write(&mut self) -> Result<(), Box<dyn Error>>;
}
pub struct SerialPort {
    port_name: String,
    baudrate: u32,
    port: Option<serial2_tokio::SerialPort>,
    parser: Option<Box<dyn IParser>>,
}
impl SerialPort {
    pub fn set_parser(&mut self, parser: Box<dyn IParser>) {
        self.parser = Some(parser);
    }
}
pub trait IParser {
    fn parse(&mut self, data: u8) -> Option<String>;
    fn parse_vec(&mut self, data: Vec<u8>) -> Option<String>;
}
