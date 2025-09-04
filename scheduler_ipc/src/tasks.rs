//! SecureIoTOS Scheduler IPC Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use cortex_m::register::psp;


/// Representation of a task in the system.
///
/// Fields:
/// - `id`: Unique task identifier.
/// - `privilege`: Privilege level of the task (e.g., 0 = unprivileged, 1 = privileged).
/// - `stack_pointer`: Pointer to the task's stack frame in memory.
#[derive(Clone)]
pub struct Task {
    pub id: u32,
    pub privilege: u8,
    pub stack_pointer: *mut u32,
}

/// Initialize example tasks for demonstration purposes.
///
/// Returns a vector containing two tasks with placeholder stack pointers.
/// In a real system, stack pointers would be set to valid memory regions
/// allocated for each task.
pub fn init_tasks() -> Vec<Task> {
    vec![
        Task { id: 0, privilege: 0, stack_pointer: 0 as *mut u32 },
        Task { id: 1, privilege: 1, stack_pointer: 0 as *mut u32 },
    ]
}

/// Perform a context switch between two tasks.
///
/// This function is responsible for saving the CPU state of the currently
/// running task and restoring the state of the next task to be executed.
/// In this example, the functions are placeholders and do not yet manipulate
/// registers or memory.
pub fn context_switch(current: &Task, next: &Task) {
    save_cpu_state(current);
    restore_cpu_state(next);
}

/// Save the CPU state of a task by updating its saved stack pointer.
///
/// In a real scheduler, this function would:
/// - Push registers onto the task's stack.
/// - Save the Process Stack Pointer (PSP) value.
/// 
/// Here, we simulate this by reading PSP and storing it in the Task struct.
fn save_cpu_state(task: &mut Task) {
    unsafe {
        // Read Process Stack Pointer (PSP)
        let current_sp = psp::read();
        task.stack_pointer = current_sp as *mut u32;

		// push registers onto the stack (R4–R11 at minimum for cooperative multitasking)
        asm!(
             "mrs {0}, psp", out(reg) current_sp,
             "stmdb {0}!, {{r4-r11}}", // push callee-saved regs
             inout(reg) current_sp => _,
         );
    }
}

/// Restore the CPU state of a task.
///
/// In a complete implementation, this would pop registers and status
/// information from the task’s stack and update the CPU to resume execution.
fn restore_cpu_state(_task: &Task) {}
