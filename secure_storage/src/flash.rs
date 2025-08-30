//! SecureIoTOS Flash Encryption Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::key_mgmt;
use crate::wear_level;

/// Encrypt and store a slice of data into flash (one logical sector).
///
/// This function:
///  - obtains an encryption key via key_mgmt (or hardware key)
///  - derives an IV (simple per-sector IV from wear-leveling index or RNG)
///  - encrypts using AES (delegates to crypto::aes)
///  - writes the ciphertext to the next sector via wear-leveling
pub fn encrypt_and_store(data: &[u8]) -> Result<(), &'static str> {
    // Get key (hardware-backed preferred)
    let key = key_mgmt::get_encryption_key();

    // Get sector index / IV from wear-leveling module
    let sector_idx = wear_level::get_next_sector_index();
    let iv = wear_level::derive_iv_for_sector(sector_idx);

    // Use the project's crypto AES function
    let ciphertext = crate::crypto::aes::encrypt_aes(data, &key, &iv);

    // Write via wear-leveling (atomic swap)
    wear_level::write_sector(sector_idx, &ciphertext)?;

    Ok(())
}

/// Read+decrypt the most recent logical sector (returns plaintext)
pub fn read_and_decrypt() -> Result<Vec<u8>, &'static str> {
    let key = key_mgmt::get_encryption_key();

    let sector_idx = wear_level::get_active_sector_index();
    let iv = wear_level::derive_iv_for_sector(sector_idx);

    let ciphertext = wear_level::read_sector(sector_idx)?;

    let plaintext = crate::crypto::aes::decrypt_aes(&ciphertext, &key, &iv);

    Ok(plaintext)
}
