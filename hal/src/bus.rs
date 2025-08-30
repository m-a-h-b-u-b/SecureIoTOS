//! SecureIoTOS HAL Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// SPI trait abstraction
pub trait Spi {
    fn write(&mut self, data: u8);
    fn read(&self) -> u8;
}

/// I2C trait abstraction
pub trait I2c {
    fn write(&mut self, addr: u8, data: &[u8]);
    fn read(&self, addr: u8, buffer: &mut [u8]);
}

/// Initialize communication buses
pub fn init_bus() {
    // Setup default SPI/I2C peripherals
}
