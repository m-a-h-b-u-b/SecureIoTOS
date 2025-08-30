//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::hal::gpio::GPIO;
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

pub struct SecureSensor {
    pub pin: GPIO,
}

static SENSOR_STATE: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

impl SecureSensor {
    pub fn new(pin: GPIO) -> Self {
        Self { pin }
    }

    /// Read sensor state safely
    pub fn read_sensor(&self) -> bool {
        cortex_m::interrupt::free(|cs| {
            *SENSOR_STATE.borrow(cs).borrow()
        })
    }

    /// Write sensor state safely
    pub fn write_sensor(&self, value: bool) {
        cortex_m::interrupt::free(|cs| {
            *SENSOR_STATE.borrow(cs).borrow_mut() = value;
        });
    }
}

/// Initialize sensor module (placeholder)
pub fn init_sensor() {
    // Configure default sensors here
}
