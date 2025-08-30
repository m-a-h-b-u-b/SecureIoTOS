//! SecureIoTOS Device Driver Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::hal::bus::Spi;

/// SPI driver abstraction
pub struct SpiDriver<T: Spi> {
    spi: T,
}

impl<T: Spi> SpiDriver<T> {
    pub fn new(spi: T) -> Self {
        Self { spi }
    }

    pub fn send(&mut self, data: &[u8]) {
        for &b in data {
            self.spi.write(b);
        }
    }

    pub fn receive(&self) -> u8 {
        self.spi.read()
    }
}
