//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use sha2::{Sha256, Digest};
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

pub fn verify_firmware(firmware: &[u8], expected_hash: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(firmware);
    let result = hasher.finalize();
    result.as_slice() == expected_hash
}

pub fn verify_signature(firmware: &[u8], sig: &Signature, pub_key: &VerifyingKey) -> bool {
    pub_key.verify(firmware, sig).is_ok()
}