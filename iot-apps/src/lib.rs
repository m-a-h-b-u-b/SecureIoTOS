//! SecureIoTOS IoTApps Library Module
//! ---------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This is the main entrypoint for SecureIoTOS IoT applications.
//! It demonstrates sensor reading and telemetry transmission, including
//! a full example of collecting telemetry and transmitting it securely
//! with AES-256-GCM encryption.

pub mod hello;
pub mod sensor;
pub mod telemetry;

use log::{info, error};

/// Run a demonstration of IoT sensor + telemetry pipeline
pub async fn run_demo() -> Result<(), &'static str> {
    // Step 1: Hello World demo
    hello::hello_world();

    // Step 2: Sensor read + secure send
    info!("Starting sensor demo...");
    match sensor::read_sensor() {
        Ok(temp) => {
            sensor::send_sensor_data(&temp)?;
        }
        Err(e) => {
            error!("Sensor read failed: {}", e);
            return Err("Sensor error");
        }
    }

    // Step 3: Telemetry collection + secure transmission
    // Monitor how a system behaves in real time, detect issues early,
    // and help developers/operators improve reliability and security.
    info!("Collecting telemetry...");
    match telemetry::collect_telemetry() {
        Ok(telemetry_data) => {
            // -----------------------------------------------------------
            // Integrated Example: Securely send telemetry using AES-256-GCM
            // -----------------------------------------------------------
            // NOTE: Replace the static key with a securely stored value in production.
            let key: [u8; 32] = [0x01; 32];
            if let Err(e) = telemetry::transmit_telemetry(&telemetry_data, &key) {
                error!("Telemetry transmission failed: {}", e);
                return Err("Telemetry transmission error");
            }
        }
        Err(e) => {
            error!("Telemetry collection failed: {}", e);
            return Err("Telemetry error");
        }
    }

    info!("IoT demo pipeline executed successfully");
    Ok(())
}
