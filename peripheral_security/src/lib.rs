//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod secure_sensor;
pub mod secure_bus;

/// Initialize all peripheral security modules
/// 
/// This function sets up:
/// - Sensor-level security (e.g., authentication, encryption)
/// - Bus communication protection (e.g., secure channels, access control)
///
/// Ensures that IoT peripherals are protected against tampering
/// and unauthorized access from the very beginning of system startup.
pub fn init_peripherals() {
    secure_sensor::init_sensor();
    secure_bus::init_bus_security();
}
