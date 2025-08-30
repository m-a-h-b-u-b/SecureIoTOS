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

pub fn context_switch(current: &Task, next: &Task) {
    save_cpu_state(current);
    restore_cpu_state(next);
}

fn save_cpu_state(_task: &Task) {}
fn restore_cpu_state(_task: &Task) {}
