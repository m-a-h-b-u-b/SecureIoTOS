//! SecureIoTOS Cryptography Module
//! SecureIoTOS Authentication & Identity Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

static DEVICE_KEY: Mutex<RefCell<[u8; 16]>> = Mutex::new(RefCell::new([0u8; 16]));

/// Initialize key storage (placeholder)
pub fn init_keys() {
    // Optionally load keys from secure flash
}

/// Store device key securely
pub fn store_device_key(key: [u8; 16]) {
    cortex_m::interrupt::free(|cs| {
        *DEVICE_KEY.borrow(cs).borrow_mut() = key;
    });
}

/// Retrieve device key safely
pub fn get_device_key() -> [u8; 16] {
    cortex_m::interrupt::free(|cs| {
        *DEVICE_KEY.borrow(cs).borrow()
    })
}
