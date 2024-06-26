use self::command_parser::parse_config;

use super::*;
use crate::errors::Errors;
use command_parser::CommandType;
use log::*;
use std::error::Error;

impl Device {
    pub fn new() -> Self {
        Device {
            running: false,
            sleep: 1000.0,
            led_state: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.running {
            error!("Device already running");
            return Err(Box::new(Errors::new("Device already running")));
        }
        info!("device is running");
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.running {
            error!("Device is not running");
            return Err(Box::new(Errors::new("Device is not running")));
        }
        info!("device is stopped");
        self.running = false;
        Ok(())
    }

    pub fn config(&mut self, freq: u8, led: bool) {
        self.led_state = led;
        self.sleep = (1.0 / freq as f32) as f32 * 1000.0;
        info!("Device configes with freq: {} and led: {}", freq, led);
    }

    fn send_error(&mut self, cmd_type: CommandType, err: Box<dyn Error>) -> Option<String> {
        let msg = err.downcast_ref::<Errors>().unwrap();
        match cmd_type {
            CommandType::None => {
                error!("unable to undersand request");
            }
            CommandType::Start => {
                return Some(format!("$0,{}\n", msg.err().to_string()).to_string())
            }
            CommandType::Stop => {
                return Some(format!("$1,{}\n", msg.err().to_string()).to_string())
            }
            CommandType::Config => {
                return Some(format!("$2,{}\n", msg.err().to_string()).to_string())
            }
        }
        None
    }
    fn send_ok(&mut self, cmd_type: CommandType) -> Option<String> {
        match cmd_type {
            CommandType::None => {
                error!("unable to undersand request");
            }
            CommandType::Start => return Some("$0,OK\n".to_string()),
            CommandType::Stop => return Some("$1,OK\n".to_string()),
            CommandType::Config => return Some("$2,OK\n".to_string()),
        }
        None
    }
}

impl IReadable for Device {
    fn read(&mut self, data: String) -> Option<String> {
        let cmd = command_parser::parse_command(data.clone());
        trace!("cmd: {:#?}", cmd);
        match cmd {
            CommandType::None => {
                error!("invalid cmd");
            }
            CommandType::Start => {
                let result = self.start();
                match result {
                    Ok(_) => {
                        return self.send_ok(cmd);
                    }
                    Err(err) => return self.send_error(cmd, err),
                }
            }
            CommandType::Stop => {
                let result = self.stop();
                match result {
                    Ok(_) => {
                        return self.send_ok(cmd);
                    }
                    Err(err) => return self.send_error(cmd, err),
                }
            }
            CommandType::Config => {
                let values = parse_config(data.clone());
                match values {
                    Some(values) => {
                        self.config(values.0, values.1);
                        return self.send_ok(cmd);
                    }
                    None => {
                        return self
                            .send_error(cmd, Box::new(Errors::new("failed to parse values")));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    #[test]
    fn test_start() {
        let mut device = Device::new();
        let result = device.start();
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn test_stops_with_stop() {
        let mut device = Device::new();
        let result = device.stop();
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_stops_with_start_stop() {
        let mut device = Device::new();
        device.start().unwrap();
        let result = device.stop();
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_confit() {
        let mut device = Device::new();
        device.config(20, true);
        assert_eq!(device.led_state, true);
        assert_eq!(device.sleep, 50.0);
    }

    #[test]
    fn test_whole_proper_cmd_start() {
        let mut device = Device::new();
        let result = device.read("$0".to_string());
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "$0,OK\n");
    }
    #[test]
    fn test_whole_proper_cmd_stop() {
        let mut device = Device::new();
        let result = device.read("$1".to_string());
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "$1,Device is not running\n");
    }
    #[test]
    fn test_whole_proper_invalid_cmd() {
        let mut device = Device::new();
        let result = device.read("$invalod".to_string());
        assert_eq!(result.is_some(), false);
    }
}
