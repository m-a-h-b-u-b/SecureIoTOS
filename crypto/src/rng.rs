//! SecureIoTOS Cryptography RNG Module
//! -----------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

use rand_core::{OsRng, RngCore};

/// Initialize hardware RNG if available.
///
/// * On embedded boards, call the MCU-specific HAL here (e.g. `stm32_hal::rng_init()`).
/// * On desktop/host builds, no explicit initialization is required.
pub fn init_rng() {
    #[cfg(target_arch = "arm")]
    {
        // Example stub: replace with your MCU HAL init
        // stm32_hal::rng::init().expect("Failed to init hardware RNG");
    }

    // On non-embedded platforms, nothing to do: `OsRng` is lazy-initialized.
}

/// Generate a random 128-bit key (16 bytes) using a cryptographically
/// secure RNG. Falls back to the operating-system RNG if no hardware RNG
/// is configured.
pub fn generate_random_key() -> [u8; 16] {
    let mut key = [0u8; 16];

    // Prefer hardware RNG if you've set one up; otherwise OsRng.
    // OsRng pulls from the host OS or hardware entropy source.
    OsRng.fill_bytes(&mut key);

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_is_random_and_16_bytes() {
        let k1 = generate_random_key();
        let k2 = generate_random_key();
        assert_eq!(k1.len(), 16);
        assert_ne!(k1, k2, "Two generated keys should almost never match");
    }
}
