//! SecureIoTOS Cryptography Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod aes;
pub mod ecc;
pub mod rng;

/// Initialize cryptography modules
pub fn init_crypto() {
    rng::init_rng();
}
