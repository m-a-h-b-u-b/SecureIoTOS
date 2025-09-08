//! SecureIoTOS MPU Module
//! ----------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module configures the ARM Cortex-M MPU for
//! kernel, task stacks, and peripherals.

// gives you access to the MPU registers (Memory Protection Unit)
use cortex_m::peripheral::MPU;

// SCB (System Control Block), used here to enable memory fault exceptions.
use cortex_m::peripheral::SCB;

/// MPU region attributes
// ARM MPU regions need an access permission code.
// This enum is just a nicer way to write those bit patterns.
#[repr(u32)]
enum MpuAccess {
    PrivRW = 0b011,    // Privileged Read/Write
    UnprivRW = 0b011,  // Unprivileged Read/Write
    PrivRO = 0b110,    // Privileged Read-Only
    FullAccess = 0b111,
}

/// Configure MPU regions for kernel, tasks, and peripherals
pub fn setup_mpu() {
	// MPU::ptr() → raw pointer to MPU registers.

    let mpu = unsafe { &*MPU::ptr() };
    let scb = unsafe { &*SCB::ptr() };

    // We must disable MPU before changing its configuration, 
	// otherwise writes may be ignored.
    unsafe { mpu.ctrl.write(0) };

    // ---------------------------
    // Region 0: Kernel code (RX, privileged)
    // ---------------------------
    unsafe {
        mpu.rnr.write(0); // Region number
        mpu.rbar.write(0x0800_0000); // Flash base
        mpu.rasr.write(
            (0b101 << 1)      // Size = 512 KB (example, adjust)
            | (1 << 0)        // Enable
            | (MpuAccess::PrivRO as u32) << 24 // PrivRO → kernel code is read-only in privileged mode.
            | (0 << 28)       // XN = 0 (execution allowed)code must run from Flash
        );
    }

    // ---------------------------
    // Region 1: Kernel stack (RW, privileged)
    // ---------------------------
    unsafe {
        mpu.rnr.write(1);
        mpu.rbar.write(0x2000_0000); // SRAM base
        mpu.rasr.write(
            (0b101 << 1)      // Size = 512 KB (example)
            | (1 << 0)        // Enable
            | (MpuAccess::PrivRW as u32) << 24
            | (1 << 28)       // XN = 1 (no execution)
        );
    }

    // ---------------------------
    // Region 2: Task1 stack (RW, unprivileged)
    // ---------------------------
    unsafe {
        mpu.rnr.write(2);
        mpu.rbar.write(0x2001_0000); // Example Task1 stack base
        mpu.rasr.write(
            (0b100 << 1)      // Size = 256 KB (example)
            | (1 << 0)        // Enable
            | (MpuAccess::UnprivRW as u32) << 24
            | (1 << 28)       // XN = 1
        );
    }

    // ---------------------------
    // Region 3: Task2 stack (RW, unprivileged)
    // ---------------------------
    unsafe {
        mpu.rnr.write(3);
        mpu.rbar.write(0x2002_0000); // Example Task2 stack base
        mpu.rasr.write(
            (0b100 << 1)      // Size = 256 KB (example)
            | (1 << 0)        // Enable
            | (MpuAccess::UnprivRW as u32) << 24
            | (1 << 28)       // XN = 1
        );
    }

    // Enable MPU with default memory map for background regions disabled
    unsafe {
        mpu.ctrl.write(1 << 0 | 1 << 2); // ENABLE | PRIVDEFENA
        scb.shcsr.modify(|r| r | (1 << 16)); // Enable MemManage fault
    }
}
