//! SecureIoTOS HAL GPIO Module
//! ----------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// Representation of a GPIO pin
pub struct GPIO {
    pub port: u8,
    pub pin: u8,
}

/// GPIO trait for controlling pin state
pub trait GpioExt {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn toggle(&mut self);
}

impl GpioExt for GPIO {
    fn set_high(&mut self) {
        unsafe { core::ptr::write_volatile(self.port as *mut u32, 1 << self.pin) };
    }

    fn set_low(&mut self) {
        unsafe { core::ptr::write_volatile(self.port as *mut u32, 0 << self.pin) };
    }

    fn toggle(&mut self) {
        unsafe {
            core::ptr::write_volatile(
                self.port as *mut u32,
                core::ptr::read_volatile(self.port as *mut u32) ^ (1 << self.pin),
            );
        }
    }
}

/// Initialize GPIOs (default configuration)
pub fn init_gpio() {
    // Example: Configure some default GPIO pins
    let mut led1 = GPIO { port: 0x4800_0000 as u8, pin: 5 }; // Example: Port A, Pin 5
    let mut led2 = GPIO { port: 0x4800_0000 as u8, pin: 6 }; // Example: Port A, Pin 6
    let mut button = GPIO { port: 0x4800_0000 as u8, pin: 13 }; // Example: Port C, Pin 13

    // Initialize default states
    led1.set_low();   // Turn off LED1
    led2.set_low();   // Turn off LED2
    button.set_high(); // Enable pull-up for button (placeholder)
}
