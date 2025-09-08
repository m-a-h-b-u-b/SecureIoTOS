//! SecureIoTOS Authentication & Identity Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod key_storage;
pub mod token;

/// Initialize authentication modules
pub fn init_auth() {
    key_storage::init_keys();
    token::init_tokens();
}
