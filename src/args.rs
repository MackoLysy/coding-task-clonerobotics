use clap::Parser;

#[derive(Parser, Clone)]
// #[command(author, version, about, long_about = None)]
pub struct ArgParser {
    #[clap(long, env, default_value = "/dev/tty/USB0")]
    serial_port: String,
    #[clap(long, env, default_value = "115000")]
    baudrate: u32,
    #[clap(long, env, default_value = "localhost")]
    host: String,
    #[clap(long, env, default_value = "7100")]
    port: u32,
    #[clap(long, env, default_value = "./")]
    databse_path: String,
}
impl ArgParser {
    pub fn get_serial_port(&self) -> &String {
        &self.serial_port
    }
    pub fn get_baudrate(&self) -> u32 {
        self.baudrate
    }
    pub fn get_host(&self) -> &String {
        &self.host
    }
    pub fn get_port(&self) -> u32 {
        self.port
    }
    pub fn get_databse_path(&self) -> &String {
        &self.databse_path
    }
}
