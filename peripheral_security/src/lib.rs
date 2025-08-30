//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod secure_sensor;
pub mod secure_bus;

/// Initialize all peripheral security modules
pub fn init_peripherals() {
    secure_sensor::init_sensor();
    secure_bus::init_bus_security();
}
