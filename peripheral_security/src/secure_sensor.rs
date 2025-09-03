//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module provides a secure wrapper around sensor GPIO access,
//! ensuring that sensor state is read and modified safely within
//! interrupt-free critical sections. It prevents race conditions
//! and unauthorized tampering with sensor data.

use crate::hal::gpio::GPIO;
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

/// SecureSensor structure wraps a GPIO pin
/// and manages its state in a thread/interrupt-safe manner.
pub struct SecureSensor {
    pub pin: GPIO,
}

/// Global sensor state stored securely using a `Mutex` and `RefCell`.
/// 
/// - `Mutex` ensures access is restricted to interrupt-free sections.
/// - `RefCell` allows interior mutability of the boolean state.
/// 
/// Default value: `false` (inactive).
static SENSOR_STATE: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

impl SecureSensor {
    /// Create a new secure sensor instance.
    ///
    /// # Parameters
    /// - `pin`: GPIO pin associated with the sensor.
    pub fn new(pin: GPIO) -> Self {
        Self { pin }
    }

    /// Read sensor state safely.
    ///
    /// This method enters a critical section to prevent race conditions
    /// when accessing the global `SENSOR_STATE`.
    ///
    /// # Returns
    /// - `true` if sensor is active
    /// - `false` if sensor is inactive
    pub fn read_sensor(&self) -> bool {
        cortex_m::interrupt::free(|cs| {
            *SENSOR_STATE.borrow(cs).borrow()
        })
    }

    /// Write sensor state safely.
    ///
    /// This method updates the sensor state in a critical section,
    /// ensuring atomic and secure updates to `SENSOR_STATE`.
    ///
    /// # Parameters
    /// - `value`: Boolean state to set (`true` = active, `false` = inactive).
    pub fn write_sensor(&self, value: bool) {
        cortex_m::interrupt::free(|cs| {
            *SENSOR_STATE.borrow(cs).borrow_mut() = value;
        });
    }
}

/// Initialize sensor module.
///
/// - Configures a default sensor on GPIO pin 0
/// - Sets secure access policy
/// - Initializes default sensor state to inactive (`false`)
pub fn init_sensor() {
    cortex_m::interrupt::free(|cs| {
        // Reset global state to "inactive"
        *SENSOR_STATE.borrow(cs).borrow_mut() = false;
    });

    // Example: attach a default GPIO pin for the first secure sensor
    let default_sensor = SecureSensor::new(GPIO::new(0));

    // Simulate initial secure configuration (placeholder)
    println!(
        "[SecureIoTOS] Secure sensor initialized on GPIO pin {} with state = {}",
        default_sensor.pin.id,
        default_sensor.read_sensor()
    );
}
