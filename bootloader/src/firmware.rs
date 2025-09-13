//! SecureIoTOS Bootloader firmware Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

// Import the SHA-256 hasher and its Digest trait (provides `update` and `finalize`)
use sha2::{Sha256, Digest};

// Import ECDSA (Elliptic Curve Digital Signature Algorithm) primitives
// from the P-256 curve implementation
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};

use subtle::ConstantTimeEq;

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
	// Create a new SHA-256 context
    let mut hasher = Sha256::new();     
	// Feed firmware bytes into the hasher
    hasher.update(firmware);            
	// Compute final 32-byte hash (digest)
    let result = hasher.finalize();     

    
	// vulnerable to timing attacks 
    // result.as_slice() == expected_hash
	
	// Safe against timing attacks
	// ct_eq compares every byte, regardless of mismatch position.
	// Always takes the same time.
	// Returns a Choice (boolean-like), which you convert with .into().
    result.as_slice().ct_eq(expected_hash).into()
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
