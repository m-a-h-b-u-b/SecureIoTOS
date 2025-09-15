//! SecureIoTOS Cryptography ECC Module
//! ------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! Provides cryptographic operations for SecureIoTOS.
//! Currently supports ECC (P-256) signing.
//! In production, private keys should be stored in secure hardware (TPM, secure element) and never exposed in RAM.

// RefCell is a smart pointer type from Rust’s core library (the minimal, no-std version of std).
// Provides interior mutability—you can mutate the data it wraps even when 
// the RefCell itself is immutable, but only at runtime.
use core::cell::RefCell;

// From the cortex-m crate, specifically for ARM Cortex-M microcontrollers.
// A mutex designed for bare-metal embedded systems that disables interrupts 
// while accessing the data, ensuring critical sections are safe.
use cortex_m::interrupt::Mutex;

// These are from the p256 crate, which implements the NIST P-256 (a.k.a. secp256r1) elliptic curve:
// SigningKey --> Holds the private key used to produce ECDSA signatures.
// Signature --> Represents an actual ECDSA signature (the pair of integers (r, s)).
// signature::Signer --> A trait (from the signature crate) that defines a sign() method.
use p256::ecdsa::{SigningKey, Signature, signature::Signer};

use crate::crypto_hw::secure_element_load_key; // hypothetical module

// A cryptographically secure random number generator (RNG) from the rand_core crate 
use rand_core::OsRng;

/// Atomic, interrupt-protected storage for the signing key
// SIGNING_KEY is a global, thread-safe and interrupt-safe container 
// that starts empty and will later hold an ECDSA signing key.
// Mutex stops interrupts while we touch it (atomic access).
// RefCell lets us mutate it even though it’s a static.
// Option represents “maybe we’ve generated the key, maybe not.”
//This pattern is common in bare-metal embedded Rust to share a single hardware or cryptographic 
// resource safely across main code and interrupt handlers.
static SIGNING_KEY: Mutex<RefCell<Option<SigningKey>>> = Mutex::new(RefCell::new(None));

/// Initialize the cryptography module
/// Generates or loads a persistent signing key
pub fn init_crypto() {
    interrupt::free(|cs| {
        let mut guard = SIGNING_KEY.borrow(cs).borrow_mut();

        if guard.is_none() {
            // ---- Production key loading ----
            // Replace this call with your platform's actual SE/HSM API.
            // Example assumes it returns a 32-byte P-256 private key.
            match secure_element_load_key() {
                Ok(private_key_bytes) => {
                    // Construct a SigningKey from the raw private key bytes.
                    let key = SigningKey::from_bytes(&private_key_bytes)
                        .expect("secure element returned invalid key");
                    *guard = Some(key);
                }
                Err(e) => {
                    // Decide how to handle hardware failures:
                    // - retry,
                    // - log and halt,
                    // - or fall back to a safe degraded mode.
                    panic!("Failed to load signing key from secure element: {e}");
                }
            }
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
