//! SecureIoTOS HAL Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub struct GPIO {
    pub port: u8,
    pub pin: u8,
}

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

/// Initialize GPIOs (example placeholder)
pub fn init_gpio() {
    // Configure default pins here
}
