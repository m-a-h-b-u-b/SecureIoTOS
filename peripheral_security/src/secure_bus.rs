//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::hal::bus::{Spi, I2c};

/// Encrypt and send via SPI
pub fn encrypt_and_send<T: Spi>(spi: &mut T, data: u8, key: u8) {
    let encrypted = data ^ key; // Simple XOR encryption
    spi.write(encrypted);
}

/// Encrypt and send via I2C
pub fn encrypt_and_send_i2c<T: I2c>(i2c: &mut T, addr: u8, data: &[u8], key: u8) {
    let encrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    i2c.write(addr, &encrypted);
}

/// Initialize bus security (placeholder)
pub fn init_bus_security() {
    // Setup encryption keys or secure communication channels
}
