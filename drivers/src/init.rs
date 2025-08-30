//! SecureIoTOS Device Driver Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::gpio_driver::GpioDriver;
use crate::spi_driver::SpiDriver;
use crate::hal::{gpio::GPIO, bus::Spi};

/// Initialize all drivers
pub fn init_all() {
    // Example: Initialize GPIO driver
    let gpio_pin = GPIO { port: 0, pin: 1 };
    let _gpio_driver = GpioDriver::new(gpio_pin);

    // Example: Initialize SPI driver (pseudo SPI peripheral)
    struct MockSpi;
    impl Spi for MockSpi {
        fn write(&mut self, _data: u8) {}
        fn read(&self) -> u8 { 0 }
    }
    let _spi_driver = SpiDriver::new(MockSpi);
}
