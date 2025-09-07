//! SecureIoTOS Kernel Library Module
//! ---------------------------------
//! License: Apache 2.0
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This is the main entrypoint of SecureIoTOS. It initializes kernel
//! subsystems and starts the scheduler.  
//!
//! NOTE: This implementation assumes an ARM Cortex-M architecture.
//! Context switching relies on `SysTick` + `PendSV` exceptions, and
//! the CPU will never "return" from `kernel_start` in normal operation.

#![no_std]
#![no_main]

pub mod scheduler;
pub mod context;
pub mod syscall;
pub mod init;

/// Starts the SecureIoTOS kernel.
///
/// Initializes core subsystems, sets up the timer/interrupts, and
/// enters the scheduler loop.  
/// This function **never returns**.
///
/// # Safety
/// - Should only be called once from the reset handler.
/// - Assumes stack pointers, MPU, SysTick, and NVIC are configured.
pub fn kernel_start() -> ! {
    // 1. Initialize core kernel subsystems (MPU, NVIC, SysTick, etc.)
    init::kernel_init();

    // 2. Start the scheduler. This will request a context switch
    //    via PendSV and jump into the first task.
    scheduler::schedule();

    // 3. In a real kernel, execution should never reach here.
    //    If it does, halt safely.
    loop {
        cortex_m::asm::wfi(); // Wait-for-interrupt low-power halt
    }
}
