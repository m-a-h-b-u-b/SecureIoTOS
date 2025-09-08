//! SecureIoTOS Kernel Module
//! ----------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Task stack utilities: static stacks, fast fill, watermarking, and canary.

#![cfg_attr(not(test), no_std)]

/// Task stacks (1 KiB each). Access requires `unsafe`.
// Declared static mut because stacks are global and 
// will be mutated by the kernel 
// (not thread-safe, but acceptable in bare-metal embedded systems).
pub static mut TASK_STACK1: [u8; 1024] = [0; 1024];
pub static mut TASK_STACK2: [u8; 1024] = [0; 1024];

/// Default fill pattern used for watermarking free stack.
// Used to fill unused stack space (like painting it with a marker).
// Later, the kernel can count how much of this 
// pattern is left untouched → measure maximum stack usage.
pub const STACK_PATTERN: u8 = 0xA5;

/// Canary pattern for detecting stack overflows.
// 8-byte magic sequence.
// Written at the bottom (lowest address) of the stack.
// If the task overflows its stack, this canary gets overwritten → easy way to detect overflow.
pub const STACK_CANARY: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];

/// Fill a task stack with a pattern (for testing or initialization).
#[inline]
pub fn write_task_stack(stack: &mut [u8], data: u8) {
    unsafe { core::ptr::write_bytes(stack.as_mut_ptr(), data, stack.len()) };
}

/// Initialize a task stack with watermark + canary.
// Fills the entire stack with 0xA5.
// Writes the 8-byte STACK_CANARY at the beginning.
// Now the stack is both “watermarked” and “guarded.”
#[inline]
pub fn init_task_stack(stack: &mut [u8]) {
    write_task_stack(stack, STACK_PATTERN);
    write_canary(stack);
}

/// Write the canary at the bottom (lowest address).
#[inline]
fn write_canary(stack: &mut [u8]) {
    let n = STACK_CANARY.len();
    stack[..n].copy_from_slice(&STACK_CANARY);
}

/// Verify that the canary is intact.  
/// Panics if overwritten (indicating stack overflow).
pub fn check_canary(stack: &[u8]) {
    let n = STACK_CANARY.len();
    if &stack[..n] != &STACK_CANARY {
        panic!("Stack canary corrupted! Possible overflow detected.");
    }
}

/// Verify all registered task stacks’ canaries.
///
/// This is intended to be called periodically
/// (e.g., from scheduler tick or idle task).
///
/// # Safety
/// - Caller must ensure all `TASK_STACK*` statics are valid
///   and properly initialized with `init_task_stack()`.
pub unsafe fn check_canary_all() {
    check_canary(&TASK_STACK1);
    check_canary(&TASK_STACK2);
}

/// Return the stack "top" pointer (end of the buffer) aligned to 8 bytes.
#[inline]
pub fn stack_top_aligned(stack: &mut [u8]) -> *mut u8 {
    let end = unsafe { stack.as_mut_ptr().add(stack.len()) as usize };
    let aligned = end & !0x7;
    aligned as *mut u8
}

/// Estimate used stack bytes since last `init_task_stack()`.
#[inline]
pub fn used_stack_bytes(stack: &[u8]) -> usize {
    let untouched_from_top = stack
        .iter()
        .rev()
        .take_while(|&&b| b == STACK_PATTERN)
        .count();
    stack.len().saturating_sub(untouched_from_top)
}

/// Convenience: remaining free bytes in the stack.
#[inline]
pub fn free_stack_bytes(stack: &[u8]) -> usize {
    stack.len() - used_stack_bytes(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canary_all_ok_then_detects_overflow() {
        unsafe {
            init_task_stack(&mut TASK_STACK1);
            init_task_stack(&mut TASK_STACK2);

            // Canary intact, should not panic
            check_canary_all();

            // Overflow one stack
            TASK_STACK1[0] = 0x00;
            let result = std::panic::catch_unwind(|| check_canary_all());
            assert!(result.is_err(), "Expected canary panic on TASK_STACK1");
        }
    }
}
