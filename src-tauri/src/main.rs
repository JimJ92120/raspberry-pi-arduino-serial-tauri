// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use serialport;

mod state;
use state::State;

mod serial_connection;
use serial_connection::SerialConnection;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn read_serial(serial_connection: tauri::State<SerialConnection>) -> Vec<u8> {
//     println!("Result from read_serial");

//     let result: Vec<u8> = serial_connection.read();
//     println!("{:?}", result);

//     result
// }

#[tauri::command]
fn write_serial(serial_connection: tauri::State<SerialConnection>, action: String) {
    // let action = String::from("Hello World");

    serial_connection.write(action);
}


fn main() {
    let serial_connection = serialport::new("/dev/ttyACM0", 9600)
        .timeout(Duration::from_millis(1000))
        .open().expect("Failed to open port");
    println!("{:?}", serial_connection.name());
    //
    tauri::Builder::default()
        .manage(State::new())
        .manage(SerialConnection::new(serial_connection))
        .invoke_handler(tauri::generate_handler![greet, write_serial])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
