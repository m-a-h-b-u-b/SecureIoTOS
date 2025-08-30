//! SecureIoTOS Device Driver Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod gpio_driver;
pub mod spi_driver;
pub mod init;

/// Initialize all drivers
pub fn init_drivers() {
    init::init_all();
}
