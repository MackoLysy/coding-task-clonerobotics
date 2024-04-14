use super::*;
use crate::errors::Errors;
use core::time;
use log::*;
use sqlite::Connection;
use std::{error::Error, u32};
impl DataBase {
    pub fn new(path: String) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::open(path.to_string())?;
        Ok(DataBase {
            connection: connection,
        })
    }

    pub fn createTable(&mut self) -> Result<(), Box<dyn Error>> {
        let query = "CREATE TABLE IF NOT EXISTS device (
            id INTEGER PRIMARY KEY,
            pressure FLOAT,
            temperature FLOAT,
            velocity FLOAT,
            created TIMESTAMP
        );";
        trace!("creating table");
        match self.connection.execute(query) {
            Ok(_) => return Ok(()),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Box::new(Errors::new(err.message.unwrap().as_str())));
            }
        }
    }

    pub fn add_device(&mut self, device: &DeviceModel) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO device (pressure, temperature, velocity, created)
        VALUES (?, ?, ?, strftime('%s', 'now'))";
        let mut stmt = self.connection.prepare(query).unwrap();
        stmt.bind((1, device.get_pressure())).unwrap();
        stmt.bind((2, device.get_temperature())).unwrap();
        stmt.bind((3, device.get_velocity())).unwrap();
        match stmt.next() {
            Ok(_) => return Ok(()),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Box::new(Errors::new(err.message.unwrap().as_str())));
            }
        }
    }

    pub fn select_all(&mut self) -> Result<Vec<DeviceModel>, Box<dyn Error>> {
        let mut items: Vec<DeviceModel> = Vec::new();
        let query = "SELECT * FROM device";
        let stmt = self.connection.prepare(query).unwrap();
        for row in stmt.into_iter() {
            match row {
                Ok(row) => {
                    trace!("{:#?}", row);
                    let model = DeviceModel::new(
                        row.read::<f64, _>("pressure"),
                        row.read::<f64, _>("temperature"),
                        row.read::<f64, _>("velocity"),
                        0.0,
                    );
                    items.push(model);
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Box::new(Errors::new(err.message.unwrap().as_str())));
                }
            }
        }
        Ok(items)
    }
    pub fn select_limit(&mut self, limit: u32) -> Result<Vec<DeviceModel>, Box<dyn Error>> {
        let mut items: Vec<DeviceModel> = Vec::new();
        let query = "SELECT * FROM device ORDER BY created DESC LIMIT ?";
        let mut stmt = self.connection.prepare(query).unwrap();
        stmt.bind((1, limit as i64)).unwrap();
        for row in stmt.into_iter() {
            match row {
                Ok(row) => {
                    trace!("{:#?}", row);
                    let model = DeviceModel::new(
                        row.read::<f64, _>("pressure"),
                        row.read::<f64, _>("temperature"),
                        row.read::<f64, _>("velocity"),
                        0.0,
                    );
                    items.push(model);
                }
                Err(err) => {
                    error!("{:#?}", err);
                    return Err(Box::new(Errors::new(err.message.unwrap().as_str())));
                }
            }
        }
        Ok(items)
    }
    fn clear_db(&mut self) -> Result<(), Box<dyn Error>> {
        let query = "DROP TABLE device";
        match self.connection.execute(query) {
            Ok(_) => return Ok(()),
            Err(err) => {
                error!("{:#?}", err);
                return Err(Box::new(Errors::new(err.message.unwrap().as_str())));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataBase;
    use super::*;
    use crate::data_base::DeviceModel;
    use std::borrow::Borrow;
    #[test]
    fn create_db() {
        let db = DataBase::new(":memory".to_string());
        assert!(db.is_ok());
    }
    #[test]
    fn create_table() {
        let mut db = DataBase::new(":memory".to_string()).unwrap();
        let result = db.createTable();
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn select_2_elemets_limit() {
        let mut db = DataBase::new(":memory".to_string()).unwrap();
        db.createTable().unwrap();
        db.add_device(DeviceModel::new(10.0, 20.0, 30.0, 0.0).borrow())
            .unwrap();
        db.add_device(DeviceModel::new(10.0, 20.0, 30.0, 0.0).borrow())
            .unwrap();
        let result = db.select_limit(1);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().len(), 1);
    }
}
