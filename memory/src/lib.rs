//! SecureIoTOS Kernel Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod heap;
pub mod mpu;
pub mod stack;

/// Initialize memory subsystem: heap, MPU, stacks
pub fn memory_init() {
    heap::init_heap(0x2000_0000, 1024 * 16); // Example heap address and size
    mpu::setup_mpu();
}
