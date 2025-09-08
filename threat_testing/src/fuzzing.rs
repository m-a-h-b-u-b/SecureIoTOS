//! SecureIoTOS fuzzing Module
//! --------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source / commercial use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! # Purpose
//! Demonstrates a simple fuzzing workflow that generates random input
//! data and feeds it into a simulated parser for SecureIoTOS kernel modules.


use rand::{thread_rng, Rng};
use log::{info, debug};

/// Runs a simple fuzzing demonstration on a dummy input parser.
///
/// # Arguments
/// - `iterations`: number of fuzzing rounds
/// - `input_size`: number of random bytes per iteration
///
/// # Notes
/// Replace the `simulate_parser` function with your actual parser/validator.
pub async fn run_fuzz_example(iterations: usize, input_size: usize) {
    info!(
        "Starting fuzzing demo: {} iterations, {} bytes each",
        iterations, input_size
    );

    for round in 1..=iterations {
        let input: Vec<u8> = (0..input_size)
            .map(|_| thread_rng().gen())
            .collect();

        debug!("Round {}: Generated input {:?}", round, input);

        match simulate_parser(&input).await {
            Ok(_) => info!("Round {} passed ", round),
            Err(e) => info!("Round {} failed  with error: {}", round, e),
        }
    }

    info!("Fuzzing simulation complete");
}

/// Dummy async parser that randomly "accepts" or "rejects" input.
///
/// Replace this with real parsing logic.
async fn simulate_parser(input: &[u8]) -> Result<(), String> {
    // Example: reject input if it contains too many zeros
    let zero_count = input.iter().filter(|&&b| b == 0).count();
    if zero_count > input.len() / 2 {
        Err(format!("Too many zero bytes ({} of {})", zero_count, input.len()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_fuzz_demo_runs() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            run_fuzz_example(3, 8).await;
        });
    }

    #[test]
    fn test_parser_rejects_zero_heavy_input() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let input = vec![0u8; 10];
            let result = simulate_parser(&input).await;
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_parser_accepts_mixed_input() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let input = vec![1, 2, 3, 0, 4, 5];
            let result = simulate_parser(&input).await;
            assert!(result.is_ok());
        });
    }
}
