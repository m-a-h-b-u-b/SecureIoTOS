//! SecureIoTOS Secure Communication libs Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! # Overview
//! This module provides secure communication primitives for IoT systems,
//! including TLS, MQTT, and CoAP protocols.

pub mod tls;
pub mod mqtt;
pub mod coap;

/// Runs a demo showcasing all available secure communication modules.
///
/// # Example
/// ```ignore
/// use secureiotos::comm::run_demo;
/// 
/// #[tokio::main]
/// async fn main() {
///     if let Err(e) = run_demo().await {
///         eprintln!("Demo failed: {}", e);
///     }
/// }
/// ```
pub async fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting SecureIoTOS Communication Demo...");

    // TLS demo
    println!("Connecting via TLS...");
    tls::connect_tls("example.com:443").await?;
    println!("TLS connection successful.");

    // MQTT demo
    println!("Publishing MQTT demo message...");
    mqtt::publish_demo().await?;
    println!("MQTT demo message published.");

    // CoAP demo
    println!("Sending CoAP demo request...");
    coap::send_demo().await?;
    println!("CoAP demo request completed.");

    println!("SecureIoTOS Communication Demo finished successfully.");
    Ok(())
}
