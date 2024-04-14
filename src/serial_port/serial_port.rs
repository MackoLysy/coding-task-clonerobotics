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
            readable: None,
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
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Reading {}", self.port_name);
        if self.port.is_none() {
            return Err(Box::new(Errors::new("Port is not setted")));
        }
        let port = self.port.as_ref().unwrap();
        let mut buff = [0];
        loop {
            match port.read(&mut buff).await {
                Ok(_) => match self.parser.clone() {
                    Some(parser) => {
                        let val = parser.as_ref().lock().unwrap().parse(buff[0]);
                        if val.is_some() {
                            let readable = self.readable.clone();
                            match readable {
                                Some(r) => {
                                    let mut guard = r.lock().unwrap();
                                    let val = guard.read(val.unwrap());
                                    if val.is_some() {
                                        match port.write_all(val.unwrap().as_bytes()).await {
                                            Ok(_) => {
                                                trace!("data sended!")
                                            }
                                            Err(_) => error!("faield to send data"),
                                        }
                                    }
                                }
                                None => {
                                    error!("Readable is not setted!");
                                    return Err(Box::new(Errors::new("parser is not setted!")));
                                }
                            }
                        }
                    }
                    None => {
                        error!("Parser is not setted!");
                        return Err(Box::new(Errors::new("parser is not setted!")));
                    }
                },
                Err(_) => {
                    error!("failed to read buffer!");
                }
            }
        }
    }
}
impl IWrittable for SerialPort {
    fn write(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
