//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;

#[entry]
fn main() -> ! {
    init_nvic();
    init_systick();

    let fw_valid = verify_firmware(unsafe {
        core::slice::from_raw_parts(0x08004000 as *const u8, 64*1024)
    }, EXPECTED_HASH);

    if !fw_valid { loop {} }

    unsafe { cortex_m::register::CONTROL.write(1); }

    let kernel: extern "C" fn() -> ! = unsafe { core::mem::transmute(0x08004000 as *const u32) };
    kernel();
}