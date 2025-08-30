//! SecureIoTOS Secure Storage Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod flash;
pub mod wear_level;
pub mod key_mgmt;

/// Initialize secure storage subsystem
/// - init crypto (if needed)
/// - mount storage / run wear-leveling init
pub fn init_secure_storage() {
    // Ensure RNG / crypto modules initialized at system level (crypto::init_crypto())
    // Initialize wear-leveling metadata
    wear_level::init_wear_level();
}
