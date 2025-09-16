//! SecureIoTOS Thread Testing Runtime Monitoring Module
//! ----------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

use tokio::time::{sleep, Duration};
use sysinfo::{System, SystemExt, ProcessorExt};

/// Runtime CPU monitoring task
///
/// Continuously prints CPU usage at a configurable interval.
/// Uses `sysinfo` crate to fetch real CPU usage data.
pub async fn monitor_cpu(interval_secs: u64, iterations: Option<u32>) {
    let mut sys = System::new_all();
    let mut count = 0;

    println!("Starting runtime CPU monitoring...");

    loop {
        sys.refresh_cpu();
        let cpu_usage = sys.global_cpu_info().cpu_usage(); // percentage
        println!("CPU usage check {}: {:.2}%", count + 1, cpu_usage);

        count += 1;
        if let Some(max_iter) = iterations {
            if count >= max_iter {
                break;
            }
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }

    println!("CPU monitoring completed after {} checks.", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor_cpu_runs() {
        // Run 2 iterations at 1-second intervals to verify function works
        monitor_cpu(1, Some(2)).await;
    }
}
