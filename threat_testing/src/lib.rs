//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod static_analysis;
pub mod fuzzing;
pub mod runtime_monitor;

/// Run a demo of threat testing utilities
pub async fn run_demo() {

    static_analysis::check_firmware_integrity();
    fuzzing::run_fuzz_example().await;
    runtime_monitor::monitor_cpu().await;
}
