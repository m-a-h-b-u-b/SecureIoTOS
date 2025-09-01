//! SecureIoTOS Bootloader Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

// Import the SHA-256 hasher and its Digest trait (provides `update` and `finalize`)
use sha2::{Sha256, Digest};

// Import ECDSA (Elliptic Curve Digital Signature Algorithm) primitives
// from the P-256 curve implementation
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

/// Verify the integrity of the firmware by comparing its SHA-256 hash
/// with the expected hash provided by a trusted source (e.g., secure server).
///
/// # Arguments
/// * `firmware` - Raw firmware binary data
/// * `expected_hash` - Trusted SHA-256 hash (32 bytes) of the firmware
///
/// # Returns
/// * `true` if firmware hash matches `expected_hash`
/// * `false` if mismatch (corrupted or tampered firmware)
pub fn verify_firmware(firmware: &[u8], expected_hash: &[u8]) -> bool {
    let mut hasher = Sha256::new();     // Create a new SHA-256 context
    hasher.update(firmware);            // Feed firmware bytes into the hasher
    let result = hasher.finalize();     // Compute final 32-byte hash (digest)

    // NOTE: This uses a direct equality check which may be vulnerable
    // to timing attacks. Consider using a constant-time comparison instead.
    result.as_slice() == expected_hash
}

/// Verify that the firmware was signed by a trusted source using ECDSA (P-256).
///
/// # Arguments
/// * `firmware` - Raw firmware binary data
/// * `sig` - ECDSA signature to validate
/// * `pub_key` - Public key of the signer (vendor or trusted authority)
///
/// # Returns
/// * `true` if signature is valid (firmware is authentic)
/// * `false` if invalid (forged or untrusted firmware)
pub fn verify_signature(firmware: &[u8], sig: &Signature, pub_key: &VerifyingKey) -> bool {
    // Uses the `Verifier` trait implementation on `VerifyingKey`
    // Returns `Ok(())` if signature is valid, error otherwise
    pub_key.verify(firmware, sig).is_ok()
}
