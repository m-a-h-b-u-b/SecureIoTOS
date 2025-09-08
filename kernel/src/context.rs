//! SecureIoTOS Kernel Context Module
//! ---------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module manages task contexts (register state, privilege level,
//! and stack pointers). It provides `context_switch`, which saves the
//! current CPU state and restores the next task’s CPU state.
//!
//! NOTE: This is Cortex-M specific and requires `unsafe` assembly
//! for manipulating registers like PSP/MSP and general-purpose registers.

/// Representation of a task in the system.
///
/// Each task has:
/// - `id`: unique identifier.
/// - `privilege`: 0 = kernel, 1 = user.
/// - `stack_pointer`: saved stack pointer for context switching.
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub privilege: u8,
    pub stack_pointer: *mut u32,
}

/// Performs a context switch between two tasks.
///
/// # Safety
/// This involves `unsafe` assembly for low-level register operations.
/// Interrupts should be disabled during the switch.
pub fn context_switch(current: &mut Task, next: &Task) {
    unsafe {
        save_cpu_state(current);
        restore_cpu_state(next);
    }
}

/// Save CPU registers and update the task's stack pointer.
///
/// On ARM Cortex-M, the hardware automatically saves some registers
/// (R0-R3, R12, LR, PC, xPSR) on exception entry.  
/// This function saves callee-saved registers (R4-R11).
unsafe fn save_cpu_state(task: &mut Task) {
    let sp: *mut u32;
    core::arch::asm!(
        "
        mrs {0}, psp          // Read Process Stack Pointer
        stmdb {0}!, {{r4-r11}} // Push R4-R11 onto stack
        ",
        out(reg) sp,
        options(nostack)
    );
    task.stack_pointer = sp;
}

/// Restore CPU registers from the task's stack pointer.
///
/// Restores R4-R11 and updates the PSP (Process Stack Pointer).
unsafe fn restore_cpu_state(task: &Task) {
    let sp = task.stack_pointer;
    core::arch::asm!(
        "
        ldmia {0}!, {{r4-r11}} // Pop R4-R11 from stack
        msr psp, {0}           // Restore PSP
        ",
        inout(reg) (sp) => _,
        options(nostack)
    );
}
