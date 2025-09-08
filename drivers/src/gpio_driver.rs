//! SecureIoTOS GPIO Driver Module
//! --------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Provides a safe and interrupt-aware GPIO driver for SecureIoTOS.
//! Supports hardware interrupts and ISR-safe operations.

use crate::hal::gpio::{GPIO, GpioExt};
use cortex_m::interrupt;

/// GPIO driver with interrupt-safe operations
pub struct GpioDriver {
    pin: GPIO,
}

impl GpioDriver {
    /// Create a new GPIO driver for the given pin
    pub fn new(pin: GPIO) -> Self {
        Self { pin }
    }

    /// Enable GPIO hardware interrupt
    ///
    /// # Safety
    /// This should configure hardware registers to enable interrupts on the pin.
    /// The actual implementation is platform-specific.
    pub fn enable_interrupt(&self) {
        // TODO: Implement hardware-specific interrupt enable
        // Example: self.pin.enable_interrupt();
    }

    /// ISR handler (called on GPIO interrupt)
    ///
    /// This function safely toggles the pin state inside a critical section,
    /// ensuring no race conditions occur during interrupt handling.
    pub fn isr_handler(&self) {
        interrupt::free(|_| {
            // Toggle pin safely in interrupt context
            self.pin.toggle();
        });
    }

    /// Write a logical level to the pin safely
    pub fn write(&self, high: bool) {
        interrupt::free(|_| {
            if high {
                self.pin.set_high();
            } else {
                self.pin.set_low();
            }
        });
    }

    /// Read the current logical level from the pin safely
    pub fn read(&self) -> bool {
        interrupt::free(|_| self.pin.is_high())
    }
}
