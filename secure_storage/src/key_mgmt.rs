//! SecureIoTOS Key Management Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use crate::crypto::rng;

/// This module manages encryption keys for flash and other secure data.
/// Keys should be hardware-backed in production (secure element, OTP fuses).
/// Here we use an in-RAM protected store (via interrupt mutex) for demo/testing.

static ENCRYPTION_KEY: Mutex<RefCell<[u8; 16]>> = Mutex::new(RefCell::new([0u8; 16]));

/// Initialize key material (call during boot once)
pub fn init_keys() {
    let key = rng::generate_random_key();
    store_encryption_key(key);
}

/// Store the encryption key (atomic protected)
pub fn store_encryption_key(key: [u8; 16]) {
    cortex_m::interrupt::free(|cs| {
        *ENCRYPTION_KEY.borrow(cs).borrow_mut() = key;
    });
}

/// Retrieve the encryption key copy (returns array)
pub fn get_encryption_key() -> [u8; 16] {
    cortex_m::interrupt::free(|cs| {
        *ENCRYPTION_KEY.borrow(cs).borrow()
    })
}

/// Rotate the key (generate a new key and optionally re-encrypt stored sectors)
pub fn rotate_key() {
    let new_key = rng::generate_random_key();
    // In production: re-encrypt stored flash sectors with new key (one-by-one, atomic)
    store_encryption_key(new_key);
}
