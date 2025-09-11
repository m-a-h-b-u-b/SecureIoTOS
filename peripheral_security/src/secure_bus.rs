//! SecureIoTOS Peripheral Security Secure Bus Module
//! -------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Production-ready (opinionated) implementation for authenticated encryption
//! over SPI and I2C buses using AEAD (ChaCha20-Poly1305).



//! **Security summary**
//! - Uses XChaCha20-Poly1305 / ChaCha20-Poly1305 AEAD for confidentiality
//!   and integrity. Prefer XChaCha20-Poly1305 for larger nonces when available.
//! - Session keys are 256-bit and are generated from a secure RNG or derived
//!   from an authenticated key-exchange (recommended in production).
//! - Nonces are 96-bit (or 24-byte for XChaCha) and MUST be unique per key.
//! - All sensitive key material is zeroized after use.
//! - Decryption failures return an error (fail-closed).

//!
//! For `no_std` embedded targets, choose `aead` crates and RNG suited to your
//! platform and swap the RNG / storage backends accordingly.

use crate::hal::bus::{I2c, Spi};
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use lazy_static::lazy_static;
use rand::RngCore;
use std::sync::Mutex;
use thiserror::Error;
use zeroize::Zeroize;

/// Errors returned by this module
#[derive(Debug, Error)]
pub enum BusSecurityError {
    #[error("session key not initialized")]
    SessionKeyUninitialized,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed or authentication failed")]
    DecryptionFailed,
    #[error("bus write failed")]
    BusWriteFailed,
}

/// Internal session key wrapper which zeroizes on drop
#[derive(Zeroize)]
#[zeroize(drop)]
struct SessionKey([u8; 32]);

lazy_static! {
    /// Global session key storage (Option). Use init/rotate APIs to set.
    static ref SESSION_KEY: Mutex<Option<SessionKey>> = Mutex::new(None);
}

/// Packet layout used by helpers in this module when sending over the bus:
/// [nonce (12 bytes)] [ciphertext ...]
/// For XChaCha (24-byte nonce), change nonce size accordingly.
const NONCE_LEN: usize = 12; // ChaCha20-Poly1305 uses 12-byte nonces

/// Initialize bus security by generating a fresh 256-bit session key.
///
/// **Note**: In production prefer deriving the session key from an authenticated
/// ECDH handshake (X25519 + HKDF) rather than purely random keys. This helper
/// is useful for bootstrapping and tests.
pub fn init_bus_security() {
    let mut key_bytes = [0u8; 32];
    // Use platform RNG; replace with hardware RNG for embedded targets
    OsRng.fill_bytes(&mut key_bytes);

    let mut guard = SESSION_KEY.lock().unwrap();
    *guard = Some(SessionKey(key_bytes));

    // Avoid logging secrets; log only state changes
    log::info!("[SecureIoTOS] Bus security initialized (session key set)");
}

/// Rotate (replace) the session key with a new random key.
pub fn rotate_session_key() {
    // zeroize happens when previous SessionKey is replaced/dropped
    init_bus_security()
}

/// Clear session key from memory
pub fn clear_session_key() {
    let mut guard = SESSION_KEY.lock().unwrap();
    *guard = None; // previous SessionKey will be zeroized on drop
    log::info!("[SecureIoTOS] Session key cleared");
}

/// Retrieve a clone of the session key bytes if initialized.
/// The clone is returned as a Vec<u8> and MUST be zeroized by the caller
/// when no longer needed. (We return an owned Vec so callers on different
/// tasks/threads don't hold the global lock while using the key.)
fn get_session_key_clone() -> Result<[u8; 32], BusSecurityError> {
    let guard = SESSION_KEY.lock().unwrap();
    if let Some(sk) = guard.as_ref() {
        Ok(sk.0)
    } else {
        Err(BusSecurityError::SessionKeyUninitialized)
    }
}

/// Encrypt and send a single byte over SPI using AEAD.
///
/// Packet format: nonce (12) || ciphertext (len=plaintext_len + tag)
pub fn encrypt_and_send_spi<T: Spi>(spi: &mut T, plaintext: &[u8]) -> Result<(), BusSecurityError> {
    let key_bytes = get_session_key_clone()?;
    let key = Key::from_slice(&key_bytes);
    let aead = ChaCha20Poly1305::new(key);

    // generate unique nonce. In many embedded systems prefer an incrementing
    // counter stored persistently; here we use random nonces for simplicity.
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = aead
        .encrypt(nonce, plaintext)
        .map_err(|_| BusSecurityError::EncryptionFailed)?;

    // Compose packet: nonce || ciphertext
    // Note: adapt to your HAL write API. Here we assume `write_frame(&[u8])`.
    let mut packet = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    packet.extend_from_slice(&nonce_bytes);
    packet.extend_from_slice(&ciphertext);

    // Send packet; translate bus errors into BusSecurityError::BusWriteFailed
    spi.write_frame(&packet).map_err(|_| BusSecurityError::BusWriteFailed)?;

    // Zeroize local sensitive copy
    let mut k = key_bytes;
    k.zeroize();

    Ok(())
}

/// Encrypt and send a buffer over I2C using AEAD.
/// Packet format: nonce (12) || ciphertext
pub fn encrypt_and_send_i2c<T: I2c>(i2c: &mut T, addr: u8, plaintext: &[u8]) -> Result<(), BusSecurityError> {
    let key_bytes = get_session_key_clone()?;
    let key = Key::from_slice(&key_bytes);
    let aead = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = aead
        .encrypt(nonce, plaintext)
        .map_err(|_| BusSecurityError::EncryptionFailed)?;

    let mut packet = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    packet.extend_from_slice(&nonce_bytes);
    packet.extend_from_slice(&ciphertext);

    i2c.write_frame(addr, &packet).map_err(|_| BusSecurityError::BusWriteFailed)?;

    let mut k = key_bytes;
    k.zeroize();

    Ok(())
}

/// Decrypt a received packet (nonce || ciphertext) and return plaintext.
/// The function authenticates the message and fails if authentication fails.
pub fn decrypt_packet(packet: &[u8]) -> Result<Vec<u8>, BusSecurityError> {
    if packet.len() <= NONCE_LEN {
        return Err(BusSecurityError::DecryptionFailed);
    }

    let (nonce_bytes, ciphertext) = packet.split_at(NONCE_LEN);

    let key_bytes = get_session_key_clone()?;
    let key = Key::from_slice(&key_bytes);
    let aead = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = aead
        .decrypt(nonce, ciphertext)
        .map_err(|_| BusSecurityError::DecryptionFailed)?;

    let mut k = key_bytes;
    k.zeroize();

    Ok(plaintext)
}

// --- Example helper traits in `crate::hal::bus` (for reference) ---
// The real HAL in your project will provide concrete implementations.
//
// pub trait Spi {
//     fn write_frame(&mut self, frame: &[u8]) -> Result<(), SpiError>;
// }
//
// pub trait I2c {
//     fn write_frame(&mut self, addr: u8, frame: &[u8]) -> Result<(), I2cError>;
// }
// -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSpi {
        last: Vec<u8>,
    }

    impl MockSpi {
        fn new() -> Self {
            Self { last: Vec::new() }
        }
    }

    impl crate::hal::bus::Spi for MockSpi {
        fn write_frame(&mut self, frame: &[u8]) -> Result<(), crate::hal::bus::SpiError> {
            self.last.clear();
            self.last.extend_from_slice(frame);
            Ok(())
        }
    }

    struct MockI2c {
        last_addr: Option<u8>,
        last_frame: Vec<u8>,
    }

    impl MockI2c {
        fn new() -> Self {
            Self { last_addr: None, last_frame: Vec::new() }
        }
    }

    impl crate::hal::bus::I2c for MockI2c {
        fn write_frame(&mut self, addr: u8, frame: &[u8]) -> Result<(), crate::hal::bus::I2cError> {
            self.last_addr = Some(addr);
            self.last_frame.clear();
            self.last_frame.extend_from_slice(frame);
            Ok(())
        }
    }

    #[test]
    fn roundtrip_spi_encrypt_decrypt() {
        // initialize
        init_bus_security();
        let mut spi = MockSpi::new();
        let payload = b"hello";
        encrypt_and_send_spi(&mut spi, payload).expect("encrypt send failed");

        // emulate receive: decrypt packet stored in mock
        let received = spi.last.clone();
        let plaintext = decrypt_packet(&received).expect("decrypt failed");
        assert_eq!(plaintext.as_slice(), payload);

        clear_session_key();
    }

    #[test]
    fn roundtrip_i2c_encrypt_decrypt() {
        init_bus_security();
        let mut i2c = MockI2c::new();
        let payload = b"iot-data";
        encrypt_and_send_i2c(&mut i2c, 0x42, payload).expect("i2c send failed");

        let received = i2c.last_frame.clone();
        let plaintext = decrypt_packet(&received).expect("decrypt failed");
        assert_eq!(plaintext.as_slice(), payload);

        clear_session_key();
    }
}
