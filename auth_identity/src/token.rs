//! SecureIoTOS Authentication & Identity Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use p256::ecdsa::{SigningKey, Signature, signature::Signer};

/// Initialize token module (placeholder)
pub fn init_tokens() {
    // Optionally pre-generate tokens
}

/// Generate a device token using ECC
pub fn generate_device_token(device_id: u32) -> Signature {
    let key = SigningKey::random(&mut rand::thread_rng());
    let message = device_id.to_be_bytes();
    key.sign(&message)
}
