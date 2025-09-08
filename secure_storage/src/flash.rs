//! SecureIoTOS Flash Encryption Module
//! -----------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source / commercial use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Provides sector-level flash encryption and secure wear-leveling integration.

// Bring in the project's key management module (handles encryption keys)
use crate::key_mgmt;

// Bring in the wear-leveling module (manages flash memory sectors fairly)
use crate::wear_level;

// Bring in `anyhow` for ergonomic error handling:
// - `Result` is a flexible error-aware return type
// - `Context` lets you add human-readable context to errors
use anyhow::{Context, Result};

/// Encrypts and securely stores a slice of data into flash.
///
/// # Process
/// 1. Fetches encryption key from [`key_mgmt`] (hardware key if available).
/// 2. Derives per-sector IV from wear-leveling index.
/// 3. Encrypts data via AES helper in [`crate::crypto::aes`].
/// 4. Writes ciphertext to flash sector using wear-leveling.
///
/// # Errors
/// Returns error if sector write fails or key retrieval fails.
///
/// # Example
/// ```ignore
/// let data = b"SecureIoTOS config block";
/// encrypt_and_store(data).expect("Flash write failed");
/// ```
pub fn encrypt_and_store(data: &[u8]) -> Result<()> {
    // Fetch encryption key
    let key = key_mgmt::get_encryption_key()
        .context("Failed to obtain encryption key")?;

    // Derive sector index + IV
    let sector_idx = wear_level::get_next_sector_index();
    let iv = wear_level::derive_iv_for_sector(sector_idx);

    // Encrypt data
    let ciphertext = crate::crypto::aes::encrypt_aes(data, &key, &iv)
        .context("AES encryption failed")?;

    // Write to flash (atomic swap via wear leveling)
    wear_level::write_sector(sector_idx, &ciphertext)
        .with_context(|| format!("Failed to write sector {}", sector_idx))?;

    Ok(())
}

/// Reads the most recent sector, decrypts, and returns the plaintext.
///
/// # Process
/// 1. Fetches active sector index from wear-leveling.
/// 2. Retrieves encryption key and IV.
/// 3. Reads ciphertext from flash and decrypts.
///
/// # Errors
/// Returns error if read/decrypt fails.
///
/// # Example
/// ```ignore
/// let plaintext = read_and_decrypt().expect("Failed to read sector");
/// println!("Recovered data: {:?}", plaintext);
/// ```
pub fn read_and_decrypt() -> Result<Vec<u8>> {
    // Fetch encryption key
    let key = key_mgmt::get_encryption_key()
        .context("Failed to obtain encryption key")?;

    // Derive sector index + IV
    let sector_idx = wear_level::get_active_sector_index();
    let iv = wear_level::derive_iv_for_sector(sector_idx);

    // Read ciphertext
    let ciphertext = wear_level::read_sector(sector_idx)
        .with_context(|| format!("Failed to read sector {}", sector_idx))?;

    // Decrypt
    let plaintext = crate::crypto::aes::decrypt_aes(&ciphertext, &key, &iv)
        .context("AES decryption failed")?;

    Ok(plaintext)
}
