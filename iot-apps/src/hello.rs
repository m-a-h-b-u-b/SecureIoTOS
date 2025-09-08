//! SecureIoTOS IoTApps Hello Module
//! --------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Provides a simple Hello World demonstration for SecureIoTOS.

use log::info;

/// Simple Hello World application
pub fn hello_world() {
    info!("Hello, SecureIoTOS!");
}

/// Configurable greeting
pub fn greet(name: Option<&str>) {
    match name {
        Some(n) => info!("Hello, {}! Welcome to SecureIoTOS ", n),
        None => hello_world(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hello() {
        hello_world();
        // No panic means success (log output can be verified in integration tests)
    }

    #[test]
    fn test_custom_greet() {
        greet(Some("Mahbub"));
        greet(None);
    }
}
