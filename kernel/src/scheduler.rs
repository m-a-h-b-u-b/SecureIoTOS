//! SecureIoTOS Kernel Scheduler Module
//! -----------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module implements the SecureIoTOS task scheduler.
//! Currently, a round-robin scheduling policy is used. The scheduler
//! selects the next runnable task and performs a context switch.
//!
//! NOTE: This implementation assumes an ARM Cortex-M architecture.
//! Context switches should normally be triggered from the PendSV
//! exception, not directly from application code.

use crate::context::{context_switch, Task};
use crate::init::get_tasks;
use core::cell::RefCell;

/// Global scheduler state (static task table + current index).
///
/// In a real kernel this might be replaced with a proper task table
/// in kernel memory with ready/wait queues.
thread_local! {
    static TASKS: RefCell<Vec<Task>> = RefCell::new(get_tasks());
    static CURRENT_INDEX: RefCell<usize> = RefCell::new(0);
}

/// Trigger the scheduler to pick the next task.
///
/// Normally this would set the PendSV interrupt pending bit so the
/// context switch happens at exception return.
pub fn schedule() {
    trigger_pendsv();
}

/// Selects the next task and performs a context switch.
///
/// # Safety
/// Must only be called in kernel/interrupt context with interrupts disabled.
pub unsafe fn do_context_switch() {
    TASKS.with(|tasks_ref| {
        CURRENT_INDEX.with(|idx_ref| {
            let mut tasks = tasks_ref.borrow_mut();
            let mut current_index = idx_ref.borrow_mut();

            let current = &mut tasks[*current_index];

            // Round-robin: move to next task
            *current_index = (*current_index + 1) % tasks.len();
            let next = &tasks[*current_index];

            context_switch(current, next);
        });
    });
}

/// Triggers PendSV exception to request a context switch.
///
/// # Safety
/// This writes to SCB->ICSR register directly.
fn trigger_pendsv() {
    const ICSR: *mut u32 = 0xE000_ED04 as *mut u32;
    const PENDSVSET: u32 = 1 << 28;

    unsafe {
        core::ptr::write_volatile(ICSR, PENDSVSET);
    }
}
