//! SecureIoTOS Key Storage Module
//! --------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Provides secure, interrupt-safe storage for device encryption keys.
//! Keys are stored in RAM (protected by a Mutex) and should ideally be
//! persisted in secure flash or a hardware security module (HSM).

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use rand::RngCore; // optional for random key generation

/// Static in-RAM key store, protected against race conditions
static DEVICE_KEY: Mutex<RefCell<[u8; 16]>> = Mutex::new(RefCell::new([0u8; 16]));

/// Initialize key storage
///
/// In production, this could:
/// - Load keys from secure flash
/// - Or generate a random key if none exists
pub fn init_keys() {
    cortex_m::interrupt::free(|cs| {
        let mut key_ref = DEVICE_KEY.borrow(cs).borrow_mut();
        if key_ref.iter().all(|&b| b == 0) {
            // Example: generate a random AES-128 key if empty
            let mut tmp_key = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut tmp_key);
            *key_ref = tmp_key;
        }
    });
}

/// Store device key securely (overwrites old key)
pub fn store_device_key(key: [u8; 16]) {
    cortex_m::interrupt::free(|cs| {
        *DEVICE_KEY.borrow(cs).borrow_mut() = key;
    });
}

/// Retrieve a copy of the device key
pub fn get_device_key() -> [u8; 16] {
    cortex_m::interrupt::free(|cs| {
        *DEVICE_KEY.borrow(cs).borrow()
    })
}

/// Zeroize device key in RAM
///
/// Useful if you want to wipe secrets before shutdown or re-provisioning
pub fn clear_device_key() {
    cortex_m::interrupt::free(|cs| {
        *DEVICE_KEY.borrow(cs).borrow_mut() = [0u8; 16];
    });
}
