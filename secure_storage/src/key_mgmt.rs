//! SecureIoTOS Key Management Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module manages encryption keys for flash and other secure data.
//! Keys should be hardware-backed in production (secure element, OTP fuses).
//! Here we use an in-RAM protected store (via interrupt mutex) for demo/testing.

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use zeroize::Zeroize;
use crate::crypto::rng;

/// Key status for monitoring initialization and rotation
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyStatus {
    Uninitialized,
    Initialized,
}

/// Atomic, interrupt-protected storage for the encryption key
static ENCRYPTION_KEY: Mutex<RefCell<[u8; 16]>> = Mutex::new(RefCell::new([0u8; 16]));
static KEY_STATUS: Mutex<RefCell<KeyStatus>> = Mutex::new(RefCell::new(KeyStatus::Uninitialized));

/// Initialize key material (call during boot once)
pub fn init_keys() {
    let key = rng::generate_random_key();
    store_encryption_key(key);
    cortex_m::interrupt::free(|cs| {
        *KEY_STATUS.borrow(cs).borrow_mut() = KeyStatus::Initialized;
    });
}

/// Store the encryption key (atomic and zeroizes previous key)
pub fn store_encryption_key(key: [u8; 16]) {
    cortex_m::interrupt::free(|cs| {
        let mut guard = ENCRYPTION_KEY.borrow(cs).borrow_mut();
        guard.zeroize();           // Clear old key securely
        *guard = key;
    });
}

/// Retrieve a copy of the encryption key
pub fn get_encryption_key() -> [u8; 16] {
    cortex_m::interrupt::free(|cs| {
        *ENCRYPTION_KEY.borrow(cs).borrow()
    })
}

/// Retrieve the current key status
pub fn get_key_status() -> KeyStatus {
    cortex_m::interrupt::free(|cs| {
        *KEY_STATUS.borrow(cs).borrow()
    })
}

/// Rotate the key safely
/// In production: re-encrypt stored flash sectors with new key (atomic or sector-by-sector)
pub fn rotate_key() {
    let new_key = rng::generate_random_key();
    
    // TODO: Re-encrypt existing flash/data sectors here
    // Example: re_encrypt_sector(old_key, new_key);

    store_encryption_key(new_key);
}
