//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::context::Task;

pub fn kernel_init() {
    setup_stack_pointers();
    setup_mpu();
    init_systick();
    init_nvic();
}

fn setup_stack_pointers() {}
fn setup_mpu() {}
fn init_systick() {}
fn init_nvic() {}

pub fn get_tasks() -> Vec<Task> {
    vec![
        Task { id: 0, privilege: 0, stack_pointer: 0 as *mut u32 },
        Task { id: 1, privilege: 1, stack_pointer: 0 as *mut u32 },
    ]
}
