//! SecureIoTOS Cryptography Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! Provides cryptographic operations for SecureIoTOS.
//! Currently supports ECC (P-256) signing.
//! In production, private keys should be stored in secure hardware (TPM, secure element) and never exposed in RAM.

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use p256::ecdsa::{SigningKey, Signature, signature::Signer};
use rand_core::OsRng;

/// Atomic, interrupt-protected storage for the signing key
static SIGNING_KEY: Mutex<RefCell<Option<SigningKey>>> = Mutex::new(RefCell::new(None));

/// Initialize the cryptography module
/// Generates or loads a persistent signing key
pub fn init_crypto() {
    cortex_m::interrupt::free(|cs| {
        let mut guard = SIGNING_KEY.borrow(cs).borrow_mut();
        if guard.is_none() {
            // In production, load key from secure element instead of generating
            let key = SigningKey::random(&mut OsRng);
            *guard = Some(key);
        }
    });
}

/// Sign a message using ECC (P-256)
///
/// # Arguments
/// * `message` - A byte slice representing the message to sign
///
/// # Returns
/// * `Signature` - ECC signature of the message
///
/// # Security Notes
/// * Uses a persistent signing key stored in `SIGNING_KEY`.
/// * In production, the key must reside in secure hardware.
/// * The signing key never leaves protected storage.
pub fn sign_message(message: &[u8]) -> Signature {
    cortex_m::interrupt::free(|cs| {
        let guard = SIGNING_KEY.borrow(cs).borrow();
        let key = guard.as_ref().expect("Cryptography module not initialized");
        key.sign(message)
    })
}

/// Optional: Rotate the signing key (requires re-signing stored messages)
/// In production, securely rotate keys in the secure element
pub fn rotate_signing_key() {
    cortex_m::interrupt::free(|cs| {
        let mut guard = SIGNING_KEY.borrow(cs).borrow_mut();
        if let Some(old_key) = guard.take() {
            drop(old_key); // Drops and zeroizes the old key
        }
        let new_key = SigningKey::random(&mut OsRng); // Replace with hardware-backed key in production
        *guard = Some(new_key);
    });
}
