//! SecureIoTOS Peripheral Security Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::hal::bus::{Spi, I2c};
use rand::Rng;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// Global session key storage (mock implementation).
lazy_static! {
    static ref SESSION_KEY: Mutex<Option<u8>> = Mutex::new(None);
}

/// Encrypt and send a single byte of data over SPI.
///
/// # Parameters
/// - `spi`: Mutable reference to the SPI bus implementation.
/// - `data`: The plaintext data byte to be sent.
/// - `key`: A simple XOR key used for lightweight encryption.
///
/// # Security Note
/// This XOR-based encryption is a **demonstration only**.
/// For production use, replace with a secure cipher (e.g., AES, ChaCha20).
pub fn encrypt_and_send<T: Spi>(spi: &mut T, data: u8, key: u8) {
    let encrypted = data ^ key;
    spi.write(encrypted);
}

/// Encrypt and send a buffer of data over I2C.
///
/// # Parameters
/// - `i2c`: Mutable reference to the I2C bus implementation.
/// - `addr`: The I2C device address.
/// - `data`: Slice of plaintext data bytes to be encrypted and sent.
/// - `key`: A simple XOR key applied to each byte.
///
/// # Security Note
/// This is a lightweight placeholder encryption. Replace with a
/// proper cryptographic mechanism for real-world applications.
pub fn encrypt_and_send_i2c<T: I2c>(i2c: &mut T, addr: u8, data: &[u8], key: u8) {
    let encrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    i2c.write(addr, &encrypted);
}

/// Initialize bus security.
///
/// This function simulates:
/// - Generating a random session key
/// - Storing it in a global key manager
/// - Preparing for secure communication
///
/// # Example
/// ```
/// init_bus_security();
/// let key = get_session_key().unwrap();
/// ```
pub fn init_bus_security() {
    let mut rng = rand::thread_rng();
    let key: u8 = rng.gen(); // Random single-byte session key

    let mut session_key = SESSION_KEY.lock().unwrap();
    *session_key = Some(key);

    println!("[SecureIoTOS] Bus security initialized. Session key generated.");
}

/// Retrieve the current session key (for demonstration).
pub fn get_session_key() -> Option<u8> {
    let key = SESSION_KEY.lock().unwrap();
    *key
}
