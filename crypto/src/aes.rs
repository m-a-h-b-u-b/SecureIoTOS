//! SecureIoTOS Cryptography Module
//! --------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Provides AES-128 CBC encryption/decryption with PKCS7 padding.
//! Recommended for secure IoT flash storage or RAM encryption.

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use anyhow::{Context, Result};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// Encrypt data using AES-128 CBC with PKCS7 padding.
///
/// # Arguments
/// * `data` - plaintext bytes to encrypt
/// * `key`  - 16-byte AES key
/// * `iv`   - 16-byte initialization vector
///
/// # Returns
/// `Result<Vec<u8>>` containing ciphertext or error
pub fn encrypt_aes(data: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>> {
    let cipher = Aes128Cbc::new_from_slices(key, iv)
        .context("Failed to create AES-128 CBC cipher")?;
    Ok(cipher.encrypt_vec(data))
}

/// Decrypt data using AES-128 CBC with PKCS7 padding.
///
/// # Arguments
/// * `data` - ciphertext bytes to decrypt
/// * `key`  - 16-byte AES key
/// * `iv`   - 16-byte initialization vector
///
/// # Returns
/// `Result<Vec<u8>>` containing plaintext or error
pub fn decrypt_aes(data: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>> {
    let cipher = Aes128Cbc::new_from_slices(key, iv)
        .context("Failed to create AES-128 CBC cipher")?;
    cipher.decrypt_vec(data).context("AES-128 CBC decryption failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 16];
        let iv = [1u8; 16];
        let plaintext = b"SecureIoTOS Test Data";

        let ciphertext = encrypt_aes(plaintext, &key, &iv).unwrap();
        let decrypted = decrypt_aes(&ciphertext, &key, &iv).unwrap();

        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
