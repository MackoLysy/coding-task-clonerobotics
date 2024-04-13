use super::*;
use crate::errors::Errors;
use log::*;
use std::error::Error;
impl SerialPort {
    pub fn new(port_name: String, baudrate: u32) -> Self {
        SerialPort {
            port_name: port_name,
            baudrate: baudrate,
            port: None,
            parser: None,
        }
    }
    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let port = match serial2_tokio::SerialPort::open(self.port_name.to_string(), self.baudrate)
        {
            Ok(port) => port,
            Err(err) => {
                error!(
                    "Failed to open serial port {} : {:#?}",
                    self.port_name.to_string(),
                    err
                );
                return Err(Box::new(Errors::new("Failed to create open port")));
            }
        };
        self.port = Some(port);
        info!("Serial port started on port {}", self.port_name);
        Ok(())
    }
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        info!("Reading {}", self.port_name);
        if self.port.is_none() {
            return Err(Box::new(Errors::new("Port is not setted")));
        }
        let port = self.port.as_ref().unwrap();
        let mut buff = [0];
        loop {
            match port.read(&mut buff).await {
                Ok(_) => {
                    info!("reading {:?}", &buff);
                }
                Err(_) => {
                    error!("failed to read buffer!");
                }
            }
        }
    }
}

impl IWritable for SerialPort {
    async fn write(&mut self) -> Result<(), Box<dyn Error>> {
        todo!("implement it");
        Ok(())
    }
}
