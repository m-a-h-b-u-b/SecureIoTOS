//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod scheduler;
pub mod ipc;
pub mod tasks;

/// Initialize scheduler and IPC
pub fn init_system() {
    ipc::init_queue();
    tasks::init_tasks();
}
