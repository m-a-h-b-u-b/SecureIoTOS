//! SecureIoTOS Cryptography Lib Module
//! -----------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

// Re-export or declare submodules. Replace the `mod` bodies
// with your actual AES/ECC implementations or keep these
// placeholders if you’re scaffolding the library.
pub mod aes;
pub mod ecc;
pub mod rng;

/// Initialize all cryptography modules.
///
/// * Initializes the RNG so that other cryptographic
///   operations have a secure entropy source.
/// * You can extend this to initialize hardware accelerators
///   or load persistent keys for AES/ECC as needed.
pub fn init_crypto() {
    // Initialize the random number generator first.
    rng::init_rng();

    // If your AES/ECC modules need startup logic (e.g. loading
    // keys from secure element or enabling hardware engines),
    // call their init routines here:
    //
    // aes::init_aes();   // (optional)
    // ecc::init_ecc();   // (optional)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto_init_runs() {
        // Should run without panic, ensuring sub-modules link properly.
        init_crypto();
    }
}
