//! SecureIoTOS 
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// Simulated sensor reading
pub fn read_sensor() -> f32 {
    let temp = 25.0; // Simulated temperature
    println!("Sensor reading: {} °C", temp);
    temp
}

/// Placeholder function to send sensor data securely
pub fn send_sensor_data(temp: f32) {
    println!("Transmitting sensor data: {} °C", temp);
}
