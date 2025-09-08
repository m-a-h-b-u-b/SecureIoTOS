//! SecureIoTOS IoTApps Sensor Module
//! ---------------------------------
//! License : Apache 2.0
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Provides a unified interface for IoT sensor readings and secure data transmission.

use serde::{Serialize, Deserialize};
use log::{info, warn, error};

/// Trait for generic IoT sensors
pub trait Sensor {
    fn read(&self) -> Result<f32, &'static str>;
    fn name(&self) -> &'static str;
}

/// Example temperature sensor (simulated)
pub struct TemperatureSensor;

impl Sensor for TemperatureSensor {
    fn read(&self) -> Result<f32, &'static str> {
        let temp = 25.0; // Simulated
        info!("{} reading: {} °C", self.name(), temp);
        Ok(temp)
    }

    fn name(&self) -> &'static str {
        "TemperatureSensor"
    }
}

/// Sensor data packet (ready for transmission)
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub sensor: String,
    pub value: f32,
    pub unit: String,
}

/// Collect a single sensor reading into structured data
pub fn collect_sensor_data<S: Sensor>(sensor: &S) -> Result<SensorData, &'static str> {
    match sensor.read() {
        Ok(value) => Ok(SensorData {
            sensor: sensor.name().to_string(),
            value,
            unit: "°C".to_string(),
        }),
        Err(e) => {
            warn!("{} failed to read: {}", sensor.name(), e);
            Err("Sensor read error")
        }
    }
}

/// Transmit sensor data securely (stub for real crypto + network)
pub fn send_sensor_data(data: &SensorData) -> Result<(), &'static str> {
    match serde_json::to_string(data) {
        Ok(payload) => {
            // Placeholder for encryption/secure transport (TLS, DTLS, MQTT, etc.)
            info!("Transmitting securely -> {}", payload);
            Ok(())
        }
        Err(_) => {
            error!("Failed to serialize sensor data");
            Err("Serialization error")
        }
    }
}
