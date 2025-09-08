//! SecureIoTOS Peripheral Security Library Module
//! ---------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Provides initialization and management of peripheral security modules
//! for IoT devices. Includes sensor-level security and secure bus communication.

pub mod secure_sensor;
pub mod secure_bus;

/// Initialize all peripheral security modules
///
/// # Description
/// This function performs a top-level initialization of all peripheral security
/// modules, ensuring that:
/// - Sensor data is authenticated and encrypted where applicable
/// - Bus communications are protected with secure channels and access control
///
/// # Safety
/// Should be called **early during system startup** to prevent any unprotected
/// peripheral interactions.
///
/// # Example
/// ```ignore
/// SecureIoTOS::peripheral_security::init_peripherals();
/// ```
pub fn init_peripherals() {
    // Initialize sensor-level security first
    secure_sensor::init_sensor();

    // Initialize secure bus communication
    secure_bus::init_bus_security();
}

/// Optional: Allow selective initialization for testing or specialized scenarios
pub fn init_peripherals_selective(sensor: bool, bus: bool) {
    if sensor {
        secure_sensor::init_sensor();
    }
    if bus {
        secure_bus::init_bus_security();
    }
}

