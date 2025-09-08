//! SecureIoTOS Device Driver Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module serves as the central initialization point for 
//! all device drivers (GPIO, SPI, etc.) used within SecureIoTOS. 
//! It provides structured setup routines and ensures that 
//! drivers are brought online in a predictable order.

use crate::gpio_driver::GpioDriver;
use crate::spi_driver::SpiDriver;
use crate::hal::{gpio::GPIO, bus::Spi};

/// Initialize all drivers used by SecureIoTOS.
///
/// This function sets up:
/// - **GPIO driver**: Configures a sample GPIO pin (port 0, pin 1).
/// - **SPI driver**: Creates a mock SPI peripheral for demonstration.
///
/// # Example
/// ```
/// use secure_iotos::device_driver::init_all;
/// 
/// fn main() {
///     init_all(); // Bring up GPIO + SPI drivers
/// }
/// ```
///
/// # Notes
/// - The `MockSpi` struct is a placeholder for an actual SPI peripheral.
/// - Replace with board-specific HAL drivers for production use.
pub fn init_all() {
    // Example: Initialize GPIO driver
    let gpio_pin = GPIO { port: 0, pin: 1 };
    let _gpio_driver = GpioDriver::new(gpio_pin);

    // Example: Initialize SPI driver with a mock implementation
    struct MockSpi;
    impl Spi for MockSpi {
        fn write(&mut self, _data: u8) {}
        fn read(&self) -> u8 { 0 }
    }
    let _spi_driver = SpiDriver::new(MockSpi);
}