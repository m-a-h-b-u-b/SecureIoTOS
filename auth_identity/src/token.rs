//! SecureIoTOS Authentication & Identity token Module
//! --------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module handles device authentication and identity management using ECC-based tokens.
//! Device tokens are ECC signatures used for secure identification.
//! In production, keys should be stored in secure hardware (TPM, secure element) and never exposed in RAM.

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

// A cryptographically secure random number generator (RNG) from the rand_core crate 
use rand_core::OsRng;

static DEVICE_SIGNING_KEY: Mutex<RefCell<Option<SigningKey>>> = Mutex::new(RefCell::new(None));

/// Initialize the token module and optionally pre-generate persistent keys
pub fn init_tokens() {
    cortex_m::interrupt::free(|cs| {
        let mut guard = DEVICE_SIGNING_KEY.borrow(cs).borrow_mut();
        if guard.is_none() {
            // In production, load key from secure element instead of generating
            let key = SigningKey::random(&mut OsRng);
            *guard = Some(key);
        }
    });
}

/// Generate a device token using ECC (P-256)
///
/// # Arguments
/// * `device_id` - Unique identifier for the device
///
/// # Returns
/// * `Signature` - ECC signature serving as a device authentication token
///
/// # Security Notes
/// * Uses a persistent signing key stored in `DEVICE_SIGNING_KEY`.
/// * In production, this key must reside in hardware-backed storage.
/// * Token is deterministic for the same key but unique per device ID.
pub fn generate_device_token(device_id: u32) -> Signature {
    cortex_m::interrupt::free(|cs| {
        let guard = DEVICE_SIGNING_KEY.borrow(cs).borrow();
        let key = guard.as_ref().expect("Token module not initialized");
        let message = device_id.to_be_bytes();
        key.sign(&message)
    })
}

/// Optional: Rotate device key (requires re-issuing tokens)
/// In production, securely rotate keys in the secure element
pub fn rotate_device_key() {
    cortex_m::interrupt::free(|cs| {
        let mut guard = DEVICE_SIGNING_KEY.borrow(cs).borrow_mut();
        // Zeroize old key before replacing
        if let Some(old_key) = guard.take() {
            drop(old_key); // `SigningKey` drops private material safely
        }
        let new_key = SigningKey::random(&mut OsRng); // Replace with hardware-backed key in production
        *guard = Some(new_key);
    });
}
