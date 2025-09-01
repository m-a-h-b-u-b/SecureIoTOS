//! SecureIoTOS Bootloader Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

#![no_std]   // Do not link the Rust standard library (required for embedded systems)
#![no_main]  // Disable the standard `main` entry point

use cortex_m_rt::entry; // Attribute macro for defining the Cortex-M entry point
use cortex_m::asm;

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
        core::slice::from_raw_parts(0x08004000 as *const u8, 64 * 1024)
    }, EXPECTED_HASH);

    // If verification fails, enter an infinite loop as a fail-safe.
    if !fw_valid { loop {} }

    // Switch to unprivileged mode by writing to the CONTROL register.
    unsafe { cortex_m::register::CONTROL.write(1); }

    // Jump to the firmware entry point.
    // The address 0x08004000 is interpreted as a function pointer.
    let kernel: extern "C" fn() -> ! =
        unsafe { core::mem::transmute(0x08004000 as *const u32) };

    kernel(); // Transfer execution to firmware (never returns)
}
