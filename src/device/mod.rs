mod command_parser;
mod device;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

use crate::serial_port::IWrittable;
pub struct Device {
    running: bool,
    sleep: f32,
    led_state: bool,
    serialDevice: Option<Arc<Mutex<dyn IWrittable>>>,
}
impl Device {
    pub fn set_sender(&mut self, val: mpsc::Sender<String>) {}

    pub fn set_serial_device(&mut self, val: Arc<Mutex<dyn IWrittable>>) {
        self.serialDevice = Some(val);
    }
}

pub trait IReadable {
    fn read(&mut self, data: String) -> Option<String>;
}
