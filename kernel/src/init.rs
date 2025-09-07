//! SecureIoTOS Kernel Init Module
//! ------------------------------
//! License: Apache 2.0
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module is responsible for initializing low-level kernel subsystems,
//! such as stack pointers, MPU, system timer (SysTick), and interrupt
//! controller (NVIC). It also provides a list of initial tasks for the
//! scheduler to start with.

use crate::context::Task;

/// Result type for kernel initialization routines.
pub type KernelResult<T> = Result<T, InitError>;

/// Possible initialization errors.
#[derive(Debug)]
pub enum InitError {
    StackPointerSetupFailed,
    MpuSetupFailed,
    SysTickInitFailed,
    NvicInitFailed,
    TaskInitFailed,
}

/// Kernel initialization entrypoint.
///
/// # Panics
/// Panics if any critical subsystem fails to initialize.  
/// In a production kernel, you may want to *gracefully* halt instead.
pub fn kernel_init() {
    if let Err(e) = setup_stack_pointers() {
        panic!("Kernel init failed: {:?}", e);
    }

    if let Err(e) = setup_mpu() {
        panic!("Kernel init failed: {:?}", e);
    }

    if let Err(e) = init_systick() {
        panic!("Kernel init failed: {:?}", e);
    }

    if let Err(e) = init_nvic() {
        panic!("Kernel init failed: {:?}", e);
    }
}

/// Setup process stack and main stack pointers.
///
/// TODO: Implement architecture-specific stack setup.
/// For ARM Cortex-M: configure `MSP` and `PSP` registers properly.
fn setup_stack_pointers() -> KernelResult<()> {
    // Stub implementation
    // unsafe { asm!("msr MSP, {}", in(reg) msp_value); }
    Ok(())
}

/// Initialize Memory Protection Unit (MPU).
///
/// TODO: Configure memory regions, set permissions, enable MPU.
fn setup_mpu() -> KernelResult<()> {
    // Stub implementation
    Ok(())
}

/// Initialize SysTick timer.
///
/// TODO: Configure SysTick for periodic interrupts to drive the scheduler.
fn init_systick() -> KernelResult<()> {
    // Stub implementation
    Ok(())
}

/// Initialize Nested Vectored Interrupt Controller (NVIC).
///
/// TODO: Configure interrupt priorities, enable required IRQs.
fn init_nvic() -> KernelResult<()> {
    // Stub implementation
    Ok(())
}

/// Returns a static list of initial kernel tasks.
///
/// TODO: Replace with dynamic task creation from kernel config.
pub fn get_tasks() -> Vec<Task> {
    vec![
        Task {
            id: 0,
            privilege: 0, // Kernel mode
            stack_pointer: core::ptr::null_mut(),
        },
        Task {
            id: 1,
            privilege: 1, // User mode
            stack_pointer: core::ptr::null_mut(),
        },
    ]
}
