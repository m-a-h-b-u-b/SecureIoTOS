//! SecureIoTOS Cryptography Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use p256::ecdsa::{SigningKey, Signature, signature::Signer};

/// Sign a message using ECC (P-256)
pub fn sign_message(message: &[u8]) -> Signature {
    let key = SigningKey::random(&mut rand::thread_rng());
    key.sign(message)
}
