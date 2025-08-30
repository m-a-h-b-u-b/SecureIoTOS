//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// Task stacks
pub static mut TASK_STACK1: [u8; 1024] = [0; 1024];
pub static mut TASK_STACK2: [u8; 1024] = [0; 1024];

/// Fill a task stack with a pattern (for testing or initialization)
pub fn write_task_stack(stack: &mut [u8], data: u8) {
    for byte in stack.iter_mut() {
        *byte = data;
    }
}
