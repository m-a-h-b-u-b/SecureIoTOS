//! SecureIoTOS Cryptography Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub fn init_rng() {
    // Initialize hardware RNG if available
}

/// Generate a random 128-bit key
pub fn generate_random_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    for i in 0..16 {
        key[i] = rand::random::<u8>();
    }
    key
}
