//! SecureIoTOS IoTApps Library Module
//! ---------------------------------
//! License : Apache 2.0
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This is the main entrypoint for SecureIoTOS IoT applications.
//! It demonstrates sensor reading and telemetry transmission.

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
    info!("Collecting telemetry...");
    match telemetry::collect_telemetry() {
        Ok(data) => {
            telemetry::transmit_telemetry(&data)?;
        }
        Err(e) => {
            error!("Telemetry collection failed: {}", e);
            return Err("Telemetry error");
        }
    }

    info!("IoT demo pipeline executed successfully");
    Ok(())
}
