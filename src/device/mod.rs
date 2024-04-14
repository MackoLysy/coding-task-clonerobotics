mod command_parser;
mod device;

pub struct Device {
    running: bool,
    sleep: f32,
    led_state: bool,
}

pub trait IReadable {
    fn read(&mut self, data: String) -> Option<String>;
}
