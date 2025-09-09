//! SecureIoTOS Kernel Library Module
//! ---------------------------------
//! License : Dual License
//!   - Apache 2.0 for open-source / personal use
//!   - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! The kernel module is the main entrypoint of SecureIoTOS.  
//! It handles:
//! - Core subsystem initialization (MPU, NVIC, SysTick, etc.)
//! - Scheduler startup and context switching
//! - Safe halting in case of unexpected return

#![no_std]
#![no_main]

pub mod scheduler;
pub mod context;
pub mod syscall;
pub mod init;

//! # Notes
//! - Assumes ARM Cortex-M architecture
//! - Context switching relies on `SysTick` + `PendSV` exceptions
//! - `kernel_start()` **never returns** under normal operation


/// Starts the SecureIoTOS kernel.
///
/// This function:
/// 1. Initializes kernel core subsystems (MPU, NVIC, SysTick, etc.)
/// 2. Starts the scheduler and enters the first task
/// 3. Halts safely if execution ever returns (should never happen)
///
/// # Safety
/// - Must be called **only once** from the reset handler
/// - Assumes:
///   * Stack pointers are properly set
///   * Memory protection (MPU) configured if used
///   * SysTick and NVIC are configured
/// - Improper usage may cause undefined behavior
pub fn kernel_start() -> ! {
    // ---------------------------
    // 1. Initialize core subsystems
    // ---------------------------
    init::kernel_init();

    // ---------------------------
    // 2. Start the scheduler
    // ---------------------------
    // This will trigger the first context switch via PendSV
    scheduler::schedule();

    // ---------------------------
    // 3. Execution should never reach here
    // ---------------------------
    // If it does, enter a safe low-power halt
    loop {
		// WFI stands for Wait For Interrupt.
		// It halts the CPU core in a low-power state.
		// The CPU stays halted until an interrupt occurs.
		// During this time, the processor does not execute normal instructions, saving power.
		// Once an interrupt is triggered, the CPU wakes up, handles the interrupt, and then continues execution.
        cortex_m::asm::wfi(); // Wait For Interrupt
    }
}
