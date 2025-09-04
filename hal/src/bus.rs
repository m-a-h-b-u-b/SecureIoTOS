//! SecureIoTOS HAL (Hardware Abstraction Layer) Module
//! ----------------------------------------------------
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module provides trait-based abstractions for hardware 
//! communication buses such as SPI and I2C. By defining 
//! generic traits, SecureIoTOS can interact with various 
//! hardware platforms in a platform-independent manner.

use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::blocking::spi::Transfer;

/// Trait representing SPI communication functionality
pub trait Spi {
    /// Write bytes to the SPI bus
    fn write(&mut self, data: &[u8]);

    /// Transfer bytes to the SPI bus and read the response
    fn transfer(&mut self, data: &mut [u8]);
}

/// Trait representing I2C communication functionality
pub trait I2c {
    /// Write bytes to a specific I2C address
    fn write(&mut self, addr: u8, data: &[u8]);

    /// Read bytes from a specific I2C address
    fn read(&mut self, addr: u8, buffer: &mut [u8]);
}

/// SPI HAL wrapper struct
/// Encapsulates any SPI implementation from embedded-hal
pub struct HalSpi<SPI> { 
    pub spi: SPI 
}

impl<SPI, E> Spi for HalSpi<SPI>
where SPI: Transfer<u8, Error = E> 
{
    /// Write data via SPI by internally performing a transfer
    fn write(&mut self, data: &[u8]) { 
        let mut buf = data.to_vec(); 
        let _ = self.spi.transfer(&mut buf); 
    }

    /// Perform a SPI transfer, modifying the input buffer with the response
    fn transfer(&mut self, data: &mut [u8]) { 
        let _ = self.spi.transfer(data); 
    }
}

/// I2C HAL wrapper struct
/// Encapsulates any I2C implementation from embedded-hal
pub struct HalI2c<I2C> { 
    pub i2c: I2C 
}

impl<I2C, E> I2c for HalI2c<I2C>
where I2C: Write<Error=E> + Read<Error=E> 
{
    /// Write bytes to the specified I2C address
    fn write(&mut self, addr: u8, data: &[u8]) { 
        let _ = self.i2c.write(addr, data); 
    }

    /// Read bytes from the specified I2C address into the buffer
    fn read(&mut self, addr: u8, buffer: &mut [u8]) { 
        let _ = self.i2c.read(addr, buffer); 
    }
}

/// Initialize the hardware communication buses
/// Returns a tuple containing the SPI and I2C HAL wrappers
///
/// # Parameters
/// - `spi`: an SPI implementation compatible with embedded-hal
/// - `i2c`: an I2C implementation compatible with embedded-hal
///
/// # Returns
/// Tuple of `(HalSpi<SPI>, HalI2c<I2C>)`
pub fn init_bus<SPI, I2C, E1, E2>(spi: SPI, i2c: I2C) -> (HalSpi<SPI>, HalI2c<I2C>)
where SPI: Transfer<u8, Error = E1>,
      I2C: Write<Error=E2> + Read<Error=E2>,
{
    (HalSpi { spi }, HalI2c { i2c })
}
