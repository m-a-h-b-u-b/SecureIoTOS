//! SecureIoTOS Cortex-M Kernel Init Module
//! ---------------------------------------
//! License: Apache 2.0
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Platform: ARM Cortex-M (ARMv7-M style registers).
//!
//! WARNING: verify register addresses and MPU semantics for your exact core.
//! Prefer using cortex-m / cortex-m-rt crates for production. This file is
//! intended as a clear, minimal, self-contained starting point.

#![no_std]
#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};
use core::arch::asm;

/// Common result type for init functions
pub type KernelResult<T> = Result<T, InitError>;

#[derive(Debug)]
pub enum InitError {
    BadArgument,
    MpuUnavailable,
    SysTickConfigError,
    NvicConfigError,
}

/// Cortex-M System Memory Map (common addresses)
const SCS_BASE: usize = 0xE000_E000;
const SYST_CSR: *mut u32 = (SCS_BASE + 0x010) as *mut u32; // SysTick Control and Status
const SYST_RVR: *mut u32 = (SCS_BASE + 0x014) as *mut u32; // SysTick Reload Value
const SYST_CVR: *mut u32 = (SCS_BASE + 0x018) as *mut u32; // SysTick Current Value

const NVIC_ISER0: *mut u32 = (SCS_BASE + 0x100) as *mut u32; // NVIC Interrupt Set-Enable Registers (ISER0)
const NVIC_ICER0: *mut u32 = (SCS_BASE + 0x180) as *mut u32; // NVIC Interrupt Clear-Enable Registers (ICER0)

/// MPU registers (ARMv7-M style)
/// NOTE: Check vendor documentation. Some Cortex-M0 parts do not have MPU.
const MPU_BASE: usize = 0xE000_ED90;
const MPU_TYPE: *mut u32 = (MPU_BASE + 0x00) as *mut u32;
const MPU_CTRL: *mut u32 = (MPU_BASE + 0x04) as *mut u32;
const MPU_RNR: *mut u32 = (MPU_BASE + 0x08) as *mut u32;
const MPU_RBAR: *mut u32 = (MPU_BASE + 0x0C) as *mut u32;
const MPU_RASR: *mut u32 = (MPU_BASE + 0x10) as *mut u32;

/// CONTROL register flags
const CONTROL_NPRIV: u32 = 1 << 0; // Thread mode privilege (0=privileged, 1=unprivileged)
const CONTROL_SPSEL: u32 = 1 << 1; // Stack pointer selection (0=MSP, 1=PSP)

/// Public kernel init entry
pub fn kernel_init(stack_top: usize, first_task_sp: usize) {
    // 1) set MSP to stack_top (typically initial MSP)
    unsafe { set_msp(stack_top as u32) };

    // 2) setup MPU (optional)
    if let Err(e) = setup_mpu() {
        // In production kernel, decide whether to panic/halt or continue
        panic!("MPU setup failed: {:?}", e);
    }

    // 3) init SysTick for preemption (example tick: CPU_HZ/1000 -> 1ms)
    // You must provide or compute `ticks_per_tick` from your clock.
    // Example below assumes an external function `core_clock_hz()` available.
    let core_hz = unsafe { core_clock_hz() };
    let ticks = core_hz / 1000; // 1ms tick
    if let Err(e) = init_systick(ticks) {
        panic!("SysTick init failed: {:?}", e);
    }

    // 4) enable required interrupts in NVIC (example: PendSV, SVC are special)
    // Example: enable IRQ number 5 (platform dependent). For real code enable
    // the IRQs you need by number.
    if let Err(e) = init_nvic(&[5u8 /* example IRQn */]) {
        panic!("NVIC init failed: {:?}", e);
    }

    // 5) set PSP for first user task and switch to use PSP in thread mode
    unsafe {
        set_psp(first_task_sp as u32);
        switch_to_psp_unprivileged();
    }
}

/// Set Main Stack Pointer (MSP)
///
/// Safety: caller must supply a valid stack top address aligned to 8 bytes
/// and appropriate for the target privilege mode.
pub unsafe fn set_msp(msp: u32) {
    // msr MSP, r0 where r0 contains msp value
    asm!(
        "msr msp, {0}",
        in(reg) msp,
        options(nostack, preserves_flags)
    );
}

/// Set Process Stack Pointer (PSP)
pub unsafe fn set_psp(psp: u32) {
    asm!(
        "msr psp, {0}",
        in(reg) psp,
        options(nostack, preserves_flags)
    );
}

/// Switch Thread mode to use PSP and unprivileged.
///
/// This writes CONTROL = (1<<1) to select PSP and (1<<0) to set unprivileged.
/// Then uses ISB to ensure effect before continuing.
pub unsafe fn switch_to_psp_unprivileged() {
    let control: u32 = CONTROL_NPRIV | CONTROL_SPSEL;
    asm!(
        "msr CONTROL, {0}",
        "isb",
        in(reg) control,
        options(nostack, preserves_flags)
    );
}

/// Setup a very small, example MPU configuration:
/// - checks MPU presence
/// - disables MPU, configures a single region, then enables MPU (privileged default map)
///
/// This is a minimal example: adapt region sizes and attributes to your needs.
pub fn setup_mpu() -> KernelResult<()> {
    unsafe {
        // Check for MPU presence
        let mpu_type = read_volatile(MPU_TYPE);
        if mpu_type == 0 {
            return Err(InitError::MpuUnavailable);
        }

        // Disable MPU before configuring
        write_volatile(MPU_CTRL, 0);

        // Example: configure region 0 with base 0x2000_0000 (SRAM) length 128KB, full access.
        // Region sizes are encoded as (region size = (1 << (N+1))) where N is RISR size field expected by RASR.
        // This example sets region 0 to be 128KB (size encoding depends on core).
        const REGION0_BASE: u32 = 0x2000_0000;
        const REGION0_NUMBER: u32 = 0;
        // RASR fields:
        // [0] ENABLE, [1:5] SRD, [8:15] AP (access perms), [16:...] SIZE, TEX/C/B bits etc.
        // We'll prepare a simple RASR value: enable, full access (AP=0b011), SIZE= (log2(128KB)-1)
        // log2(128KB) = 17, so SIZE field = 16 (SIZE enc = region size = (1 << (SIZE+1)))
        let size_field: u32 = 16; // verify for your core
        let ap_full_access: u32 = 0b011 << 24; // position depends on core; double check
        let rasr_value: u32 = (1 << 0)           // ENABLE
            | (ap_full_access)
            | ((size_field & 0x1F) << 1);       // illustrative; verify bit layout for your core

        // Select region number
        write_volatile(MPU_RNR, REGION0_NUMBER);
        write_volatile(MPU_RBAR, REGION0_BASE);
        write_volatile(MPU_RASR, rasr_value);

        // Enable MPU with default memory map for privileged access (PRIVDEFENA bit)
        // MPU_CTRL: [0] ENABLE, [2] PRIVDEFENA
        const MPU_CTRL_ENABLE: u32 = 1;
        const MPU_CTRL_PRIVDEFENA: u32 = 1 << 2;
        write_volatile(MPU_CTRL, MPU_CTRL_ENABLE | MPU_CTRL_PRIVDEFENA);

        // Data and instruction synchronization barriers
        asm!("dsb", "isb", options(nomem, nostack, preserves_flags));
    }

    Ok(())
}

/// Initialize SysTick timer
///
/// `ticks` should be chosen such that 0 < ticks <= 0x00FF_FFFF (24-bit reload on many Cortex-M).
/// Returns error if ticks doesn't fit.
pub fn init_systick(ticks: u32) -> KernelResult<()> {
    if ticks == 0 || ticks > 0x00FF_FFFF {
        return Err(InitError::SysTickConfigError);
    }
    unsafe {
        // Disable SysTick during setup
        write_volatile(SYST_CSR, 0);
        // Set reload value
        write_volatile(SYST_RVR, ticks - 1);
        // Clear current value
        write_volatile(SYST_CVR, 0);
        // Enable SysTick: CLKSOURCE = processor clock (bit 2 = 1), TICKINT (bit 1 = 1), ENABLE (bit 0 = 1)
        const SYSTICK_CLKSOURCE_PROC: u32 = 1 << 2;
        const SYSTICK_TICKINT: u32 = 1 << 1;
        const SYSTICK_ENABLE: u32 = 1 << 0;
        write_volatile(SYST_CSR, SYSTICK_CLKSOURCE_PROC | SYSTICK_TICKINT | SYSTICK_ENABLE);
    }
    Ok(())
}

/// Enable IRQs in NVIC. `irqs` is a slice of IRQ numbers (zero-based).
pub fn init_nvic(irqs: &[u8]) -> KernelResult<()> {
    unsafe {
        for &irq in irqs {
            let index = (irq / 32) as usize;
            let bit = irq % 32;
            let iser = NVIC_ISER0.add(index);
            write_volatile(iser, 1u32 << bit);
        }
    }
    Ok(())
}

/// Helper: disable IRQs (if needed)
pub fn disable_nvic(irqs: &[u8]) -> KernelResult<()> {
    unsafe {
        for &irq in irqs {
            let index = (irq / 32) as usize;
            let bit = irq % 32;
            let icer = NVIC_ICER0.add(index);
            write_volatile(icer, 1u32 << bit);
        }
    }
    Ok(())
}

/// Placeholder for obtaining the core clock frequency in Hz.
/// Replace this with a platform-specific clock source reader.
#[allow(dead_code)]
unsafe fn core_clock_hz() -> u32 {
    // Example placeholder: 168 MHz typical for some Cortex-M4 boards (adjust)
    168_000_000u32
}
