//! SecureIoTOS Device Driver Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::hal::gpio::{GPIO, GpioExt};
use cortex_m::interrupt;

pub struct GpioDriver {
    pin: GPIO,
}

impl GpioDriver {
    pub fn new(pin: GPIO) -> Self {
        Self { pin }
    }

    /// Enable GPIO hardware interrupt
    pub fn enable_interrupt(&self) {
        // Hardware-specific interrupt enable
    }

    /// ISR handler (called on interrupt)
    pub fn isr_handler(&self) {
        interrupt::free(|_| {
            // Safe access to shared resource
            self.pin.toggle();
        });
    }
}
