use std::{sync::Mutex, str::Bytes};

use serialport::{self, SerialPort};

#[derive(Debug)]
pub struct SerialConnection {
    pub connection: Mutex<Box<dyn SerialPort>>
}
impl SerialConnection {
    pub fn new(serial_connection: Box<dyn SerialPort>) -> Self {
        SerialConnection {
            connection: Mutex::from(serial_connection)
        }
    }

    pub fn read(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        self.connection
            .lock().unwrap()
            .read(result.as_mut_slice());
        

        result
    }

    pub fn write(&self, bytes: &[u8]) {
        self.connection
            .lock().unwrap()
            .write(bytes)
            .expect("Write failed!");

    }
}
