//! SecureIoTOS Device Driver Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module provides a robust SPI driver abstraction for SecureIoTOS.
//! It wraps the HAL-level SPI implementation, supports error handling,
//! bulk transfers, and full-duplex operations.

use crate::hal::bus::{Spi, SpiError};

/// SPI driver abstraction.
///
/// Wraps a generic SPI peripheral implementing the [`Spi`] trait,
/// providing high-level `send`, `receive`, and `transfer` methods
/// with proper error handling.
///
/// # Type Parameters
/// - `T`: The underlying SPI peripheral implementation.
pub struct SpiDriver<T: Spi> {
    spi: T,
}

impl<T: Spi> SpiDriver<T> {
    /// Create a new `SpiDriver` instance.
    ///
    /// # Parameters
    /// - `spi`: The underlying SPI peripheral.
    ///
    /// # Returns
    /// A new `SpiDriver` instance.
    pub fn new(spi: T) -> Self {
        Self { spi }
    }

    /// Send a buffer of bytes over SPI.
    ///
    /// # Parameters
    /// - `data`: Slice of bytes to send.
    ///
    /// # Errors
    /// Returns `SpiError` if any write operation fails.
    pub fn send(&mut self, data: &[u8]) -> Result<(), SpiError> {
        self.spi.write_bytes(data)
    }

    /// Receive a single byte from SPI.
    ///
    /// # Errors
    /// Returns `SpiError` if the read operation fails.
    pub fn receive(&self) -> Result<u8, SpiError> {
        self.spi.read()
    }

    /// Full-duplex transfer: write `tx_buffer` and read into `rx_buffer`.
    ///
    /// # Parameters
    /// - `tx_buffer`: Slice of bytes to send.
    /// - `rx_buffer`: Mutable slice to store received bytes (must match length of `tx_buffer`).
    ///
    /// # Errors
    /// Returns `SpiError` if buffers mismatch or any SPI operation fails.
    pub fn transfer(&mut self, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> Result<(), SpiError> {
        self.spi.transfer(tx_buffer, rx_buffer)
    }
}
