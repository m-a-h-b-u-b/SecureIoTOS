//! SecureIoTOS Bootloader Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

#![no_std]   // Do not link the Rust standard library (required for embedded systems)
#![no_main]  // Disable the standard `main` entry point

use cortex_m_rt::entry; // Attribute macro for defining the Cortex-M entry point
use cortex_m::asm;

const FIRMWARE_START: u32 = 0x0800_4000;
const FIRMWARE_SIZE: usize = 64 * 1024;

/// Program entry point (executed at reset).
/// This function:
/// 1. Initializes NVIC and SysTick.
/// 2. Verifies firmware integrity.
/// 3. Transfers control to the firmware if verification passes.
#[entry]
fn main() -> ! {
    init_nvic();    // Configure Nested Vectored Interrupt Controller
    init_systick(); // Configure system tick timer

    // Load firmware directly from flash (starting at 0x08004000).
    // Assumes firmware size is 64 KB.
    let fw_valid = verify_firmware(unsafe {
        core::slice::from_raw_parts(FIRMWARE_START as *const u8, FIRMWARE_SIZE)
    }, EXPECTED_HASH);
	
	fn fail_safe() -> ! {
    // TODO: Blink error LED or reset via watchdog
    loop { cortex_m::asm::wfi(); } // low-power wait
}

    // If verification fails, enter an infinite loop as a fail-safe.
    if !fw_valid { loop { fail_safe (); } }

    // Switch to unprivileged mode by writing to the CONTROL register.
	// requires unsafe is because writing to CPU control registers 
	// is inherently unsafe in Rust’s memory and execution model.
    unsafe { cortex_m::register::CONTROL.write(1); }

    // Jump to the firmware entry point.
    // The address 0x08004000 is interpreted as a function pointer.
    let kernel: extern "C" fn() -> ! =
        unsafe { core::mem::transmute(FIRMWARE_START as *const u32) };

    kernel(); // Transfer execution to firmware (never returns)
}
