//! SecureIoTOS Kernel Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod tls;
pub mod mqtt;
pub mod coap;

/// Example function demonstrating all modules
pub async fn run_demo() {

    tls::connect_tls("example.com:443").await;
    mqtt::publish_demo().await;
    coap::send_demo().await;
}
