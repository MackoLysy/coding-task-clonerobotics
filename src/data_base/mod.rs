use sqlite::Connection;
use std::time::Duration;

mod data_base;
pub struct DataBase {
    connection: sqlite::Connection,
}

pub struct DeviceModel {
    pressure: f64,
    temperature: f64,
    velocity: f64,
    timestamp: f64,
}

impl DeviceModel {
    pub fn new(pressure: f64, temperature: f64, velocity: f64, timestamp: f64) -> Self {
        DeviceModel {
            pressure: pressure,
            temperature: temperature,
            velocity: velocity,
            timestamp: timestamp,
        }
    }
    pub fn get_pressure(&self) -> f64 {
        self.pressure
    }
    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }
    pub fn get_timestamp(&self) -> f64 {
        self.timestamp
    }
}
