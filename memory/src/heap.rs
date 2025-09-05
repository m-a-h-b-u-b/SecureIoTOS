//! SecureIoTOS HEAP Module
//! ----------------------------------------------------
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module provides kernel heap initialization and
//! memory allocation support using a global allocator.

use core::alloc::Layout;
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;

/// Global kernel heap allocator
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// A fallback allocator for handling OOM gracefully
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!(
        "Kernel heap allocation failed. Requested layout: {:?}",
        layout
    );
}

/// Initialize the kernel heap at the given memory address
///
/// # Safety
/// - `start` must be a valid pointer to a memory region
///   reserved for the kernel heap.
/// - `size` must not overlap with other critical regions
///   (stack, peripherals, MPU regions, etc).
///
/// # Example
/// ```ignore
/// // Example heap initialization with 16 KB
/// init_heap(0x2003_0000, 16 * 1024);
/// ```
pub fn init_heap(start: usize, size: usize) {
    unsafe {
        ALLOCATOR.lock().init(start, size);
    }
}

/// Allocate a test block (for debugging heap functionality)
pub fn kernel_alloc_test() {
    // Example: allocate a small array
    let vec = vec![1u8, 2, 3, 4, 5];
    // Normally, we wouldn’t print inside a kernel,
    // but this helps confirm heap works in early testing.
    cortex_m_semihosting::hprintln!("Allocated vector: {:?}", vec).ok();
}

/// Get heap stats (allocated vs free) — useful for debugging
pub fn heap_stats() -> (usize, usize) {
    let allocator = ALLOCATOR.lock();
    (allocator.size(), allocator.used())
}
