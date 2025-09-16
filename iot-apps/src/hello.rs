//! SecureIoTOS IoTApps Hello Module
//! --------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source / commercial use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! This module provides a simple Hello World demonstration
//! and a configurable greeting function for SecureIoTOS.


use log::{info, warn};

/// Logs a simple Hello World message for SecureIoTOS.
pub fn hello_world() {
    info!("Hello, SecureIoTOS!");
}

/// Logs a personalized greeting if a name is provided, otherwise defaults to [`hello_world`].
///
/// # Examples
/// ```
/// use secure_iotos::hello::greet;
/// greet(Some("Alice"));
/// greet(None);
/// ```

// name: Option<&str> – The parameter is an Option that may contain a 
// string slice (&str) or be None.
// Some(&str) → caller passed a name.
// None → caller passed no name.
pub fn greet(name: Option<&str>) {
	// match is Rust’s powerful pattern-matching construct.
    match name {
		// attern: Some(n) – executes when name is Some(&str), binding the inner string to n.
		// Guard: if !n.trim().is_empty() – an extra condition that ensures the string isn’t just whitespace.
		// Action: Logs a greeting with info!.
		// n.trim() removes leading/trailing whitespace before printing.
        Some(n) if !n.trim().is_empty() => {
            info!("Hello, {}! Welcome to SecureIoTOS ", n.trim());
        }
        Some(_) => {
            warn!("Received an empty name. Falling back to default greeting.");
            hello_world();
        }
        None => hello_world(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hello() {
        hello_world();
        // No panic means success (log output verified in integration tests)
    }

    #[test]
    fn test_custom_greet_with_name() {
        greet(Some("Mahbub"));
    }

    #[test]
    fn test_custom_greet_with_empty_name() {
        greet(Some("   ")); // should fallback to hello_world with warning
    }

    #[test]
    fn test_custom_greet_none() {
        greet(None);
    }
}
