//! SecureIoTOS
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::sensor;

/// Telemetry data structure
pub struct TelemetryData {
    pub temperature: f32,
    pub humidity: f32,
}

/// Collect telemetry data from multiple sensors
pub fn collect_telemetry() -> TelemetryData {
    TelemetryData {
        temperature: sensor::read_sensor(),
        humidity: 50.0, // Simulated humidity
    }
}

/// Transmit telemetry data securely
pub fn transmit_telemetry(data: &TelemetryData) {
    println!(
        "Transmitting telemetry -> Temperature: {}, Humidity: {}",
        data.temperature, data.humidity
    );
}
