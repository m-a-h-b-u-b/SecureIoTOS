//! SecureIoTOS Scheduler IPC Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// Core modules of SecureIoTOS
pub mod scheduler; // Task scheduler (e.g., round-robin)
pub mod ipc;       // Inter-process communication (message queue)
pub mod tasks;     // Task management (task structures and context switching)

/// Initialize the SecureIoTOS system.
///
/// This function performs basic system initialization:
/// 1. Initializes the IPC message queue.
/// 2. Initializes task structures.
///
/// Should be called once during system startup before starting
/// the scheduler or executing tasks.
pub fn init_system() {
    ipc::init_queue();
    tasks::init_tasks();
}
