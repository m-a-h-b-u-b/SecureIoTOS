//! SecureIoTOS Authentication & Identity Lib Module
//! ------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod key_storage;
pub mod token;

/// Initialize authentication modules for production.
///
/// This function ensures:
/// - Device cryptographic keys are loaded from secure storage (RAM/HSM/Flash)
/// - Authentication tokens or session management structures are initialized
/// - Ready for secure identity and auth operations
pub fn init_auth() {
    // Initialize device key storage first
    // In production, keys should come from secure flash or HSM, not generated randomly
    key_storage::init_keys();

    // Initialize authentication tokens (e.g., JWTs, session keys)
    // Token module should implement secure token generation using device keys
    token::init_tokens();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_init_runs() {
        // Ensure init_auth() completes without panic in a test environment
        init_auth();
    }
}
