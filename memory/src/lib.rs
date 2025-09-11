//! SecureIoTOS Kernel Lib Module
//! -----------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! ## Notes
//! - In Rust, `mod` declares a module (a namespace for organizing code).  
//! - By default, modules and items are **private**.  
//! - Adding `pub` makes them **publicly accessible** from outside the crate.  

// Submodules for memory management
pub mod heap;
pub mod mpu;
pub mod stack;

/// Default heap start address (example: SRAM region)
const HEAP_START: usize = 0x2000_0000;

/// Default heap size in bytes (16 KiB)
const HEAP_SIZE: usize = 16 * 1024;


/// Initialize the memory subsystem: heap, MPU, and stacks.
///
/// This function sets up:
/// - **Heap allocator** with a default region
/// - **MPU (Memory Protection Unit)** for memory safety
/// - **Stack system** (future extension)
///
/// # Examples
///
/// ```rust
/// use secure_iotos::memory::memory_init;
///
/// fn main() {
///     memory_init(); // Initialize all memory subsystems
/// }
/// ```
pub fn memory_init() {
    // Initialize heap allocator
    heap::init_heap(HEAP_START, HEAP_SIZE);

    // Configure MPU
    mpu::setup_mpu();

    // TODO: Initialize stacks if needed
    // stack::init_stacks();
}