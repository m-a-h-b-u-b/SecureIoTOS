//! SecureIoTOS HEAP Module
//! ----------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module provides kernel heap initialization and
//! memory allocation support using a global allocator.


/// Layout → describes memory allocation requests (size + alignment).
use core::alloc::Layout;
/// null_mut → unsafe null pointer
use core::ptr::null_mut;

/// LockedHeap → allocator implementation from linked_list_allocator
/// A fixed size heap backed by a linked list of free memory blocks.
/// It’s a simple allocator designed for no_std embedded systems.
/// LockedHeap provides interior mutability + synchronization.
use linked_list_allocator::LockedHeap;

#![no_std]

use core::alloc::Layout;
use core::mem::MaybeUninit;
use cortex_m::interrupt;
use cortex_m::peripheral::SCB;

/// A small struct to record OOM info for post-mortem analysis.
/// Stored in RAM at a fixed address (won't use heap).
#[repr(C)]
pub struct OomRecord {
    pub size: usize,
    pub align: usize,
    pub magic: u32, // optional marker so a post-mortem tool can detect validity
}

/// A statically reserved slot for OOM diagnostics.
/// Use MaybeUninit to avoid running constructors and to place this in .bss.
static mut OOM_RECORD: MaybeUninit<OomRecord> = MaybeUninit::uninit();

/// Optional user provided handler. Set at init time with `set_oom_handler`.
/// Unsafe global simple hook — kept minimal to avoid pulling heavy runtime deps.
static mut OOM_USER_HANDLER: Option<fn(Layout)> = None;

/// Allow kernel code to register a lightweight OOM callback (must be set before OOM can occur).
/// The callback runs while interrupts are disabled, so it must be fast and safe.
pub fn set_oom_handler(handler: fn(Layout)) {
    unsafe {
        OOM_USER_HANDLER = Some(handler);
    }
}

/// Global kernel heap allocator
/// Declares the global allocator that Rust will use for Box, Vec, String, etc.
/// Initially empty; must be initialized later via init_heap().
/// #[global_allocator] → tells Rust to use this as the default allocator.
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


/// Production-ready alloc error handler.
/// - Disables interrupts
/// - Records diagnostic info in RAM
/// - Attempts to log via available backends (feature-gated)
/// - Calls user hook if set
/// - Performs a system reset (default final action)
/// Called when allocation fails (OOM = out of memory)
/// -> ! means diverging function (never returns)
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    // 1) stop preemption and further interrupts ASAP
    interrupt::disable();

    // 2) store diagnostics to reserved RAM (safe: no heap)
    unsafe {
        let rec = OomRecord {
            size: layout.size(),
            align: layout.align(),
            magic: 0x4F4F4D21, // "OOM!" marker
        };
        OOM_RECORD.as_mut_ptr().write(rec);
    }

    // 3) best-effort logging (feature gated to keep binary small)
    // Use whichever backend is compiled in. None of these are required.
    #[cfg(feature = "defmt")]
    {
        defmt::error!("OOM: size={} align={}", layout.size(), layout.align());
    }

    #[cfg(all(not(feature = "defmt"), feature = "rtt"))]
    {
        // rtt_target is common and doesn't require semihosting
        rtt_target::rprintln!("OOM: size={} align={}", layout.size(), layout.align());
    }

    #[cfg(all(not(feature = "defmt"), not(feature = "rtt"), feature = "semihosting"))]
    {
        let _ = cortex_m_semihosting::hprintln!("OOM: size={} align={}", layout.size(), layout.align());
    }

    // 4) call lightweight user hook if present (must be quick)
    unsafe {
        if let Some(cb) = OOM_USER_HANDLER {
            // The callback runs with interrupts disabled. Keep it simple.
            cb(layout);
        }
    }

    // 5) Final controlled action: reset the MCU.
    // Alternatives: loop forever, enter low-power halt, blink LED, or jump to bootloader.
    // We choose reset because it is often the safest way to recover automatically.
    cortex_m::peripheral::SCB::sys_reset();

    // sys_reset should not return — but Rust requires a diverging function.
    loop {
        // As a last resort, just spin here if reset didn't take effect.
        cortex_m::asm::nop();
    }
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
	// Uses cortex_m_semihosting::hprintln! to print via debugger.
    cortex_m_semihosting::hprintln!("Allocated vector: {:?}", vec).ok();
}

/// Get heap stats (allocated vs free) 
// Returns (total_size, used_size) of the heap.
// Useful for debugging memory usage in the kernel.
pub fn heap_stats() -> (usize, usize) {
    let allocator = ALLOCATOR.lock();
    (allocator.size(), allocator.used())
}
