//! SecureIoTOS Wear-Leveling Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

//! Simple circular wear-leveling manager implemented in RAM metadata.
//! Production systems should persist metadata and handle power-fail atomicity.

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

// Number of physical sectors used for the logical storage
const NUM_SECTORS: usize = 4;
const SECTOR_SIZE: usize = 4096; // example sector size (bytes)

static mut PHYSICAL_FLASH: [[u8; SECTOR_SIZE]; NUM_SECTORS] = [[0u8; SECTOR_SIZE]; NUM_SECTORS];

// runtime metadata (would normally live in reserved flash area)
static ACTIVE_SECTOR: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// Initialize the wear-leveling metadata
pub fn init_wear_level() {
    cortex_m::interrupt::free(|cs| {
        *ACTIVE_SECTOR.borrow(cs).borrow_mut() = 0;
    });
}

/// Get the next physical sector index to write (circular)
pub fn get_next_sector_index() -> usize {
    cortex_m::interrupt::free(|cs| {
        let mut idx = ACTIVE_SECTOR.borrow(cs).borrow().clone();
        idx = (idx + 1) % NUM_SECTORS;
        idx
    })
}

/// Write a ciphertext into the specified sector (simulated)
pub fn write_sector(sector_idx: usize, data: &[u8]) -> Result<(), &'static str> {
    if sector_idx >= NUM_SECTORS || data.len() > SECTOR_SIZE {
        return Err("invalid sector or oversize data");
    }

    // Simulate flash erase+program (in real hardware: erase then program)
    unsafe {
        let sector = &mut PHYSICAL_FLASH[sector_idx];
        for i in 0..data.len() {
            sector[i] = data[i];
        }
    }

    // Mark sector as active (atomic in this example via interrupt-free)
    cortex_m::interrupt::free(|cs| {
        *ACTIVE_SECTOR.borrow(cs).borrow_mut() = sector_idx;
    });

    Ok(())
}

/// Read the specified sector (returns a Vec of the sector content)
pub fn read_sector(sector_idx: usize) -> Result<Vec<u8>, &'static str> {
    if sector_idx >= NUM_SECTORS { return Err("invalid sector"); }

    let mut buf = Vec::with_capacity(SECTOR_SIZE);
    unsafe {
        let sector = &PHYSICAL_FLASH[sector_idx];
        for &b in sector.iter() {
            buf.push(b);
        }
    }
    Ok(buf)
}

/// Return the active sector index (last written)
pub fn get_active_sector_index() -> usize {
    cortex_m::interrupt::free(|cs| {
        *ACTIVE_SECTOR.borrow(cs).borrow()
    })
}

/// Derive a per-sector IV from the sector index (simple deterministic method)
/// In production, prefer a random IV stored alongside the ciphertext or derived via secure KDF.
pub fn derive_iv_for_sector(sector_idx: usize) -> [u8; 16] {
    let mut iv = [0u8; 16];
    iv[0] = sector_idx as u8;
    // Fill rest with fixed or better: RNG-derived nonce saved with sector
    iv
}
