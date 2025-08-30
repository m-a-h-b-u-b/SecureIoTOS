//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

#[derive(Clone)]
pub struct Task {
    pub id: u32,
    pub privilege: u8,
    pub stack_pointer: *mut u32,
}

/// Example tasks initialization
pub fn init_tasks() -> Vec<Task> {
    vec![
        Task { id: 0, privilege: 0, stack_pointer: 0 as *mut u32 },
        Task { id: 1, privilege: 1, stack_pointer: 0 as *mut u32 },
    ]
}

/// Context switch between tasks
pub fn context_switch(current: &Task, next: &Task) {
    save_cpu_state(current);
    restore_cpu_state(next);
}

fn save_cpu_state(_task: &Task) {}
fn restore_cpu_state(_task: &Task) {}
