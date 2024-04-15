mod command_parser;
mod device;
use crate::data_base::DataBase;
use std::sync::{Arc, Mutex};

pub struct Device {
    running: bool,
    sleep: f32,
    led_state: bool,
    data_base: Option<Arc<Mutex<DataBase>>>,
}
unsafe impl Send for Device {}
impl Device {
    pub fn set_data_base(&mut self, val: Arc<Mutex<DataBase>>) {
        self.data_base = Some(val);
    }
}

pub trait IReadable {
    fn read(&mut self, data: String) -> Option<String>;
}
