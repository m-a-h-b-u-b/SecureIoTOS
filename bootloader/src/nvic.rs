//! SecureIoTOS Bootloader NVC Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

/// We use the cortex-m crate, which provides safe access to ARM Cortex-M peripherals.
/// NVIC → Nested Vectored Interrupt Controller, manages interrupts.
/// SYST → SysTick timer peripheral, used for periodic ticks.
/// SystClkSource → Enum to choose clock source for SysTick (Core clock vs external reference).
use cortex_m::peripheral::{NVIC, SYST};
use cortex_m::peripheral::syst::SystClkSource;

/// Initialize the Nested Vectored Interrupt Controller (NVIC).
///
/// This function:
/// - Sets priority grouping.
/// - Configures interrupt priorities.
/// - Enables specific interrupts as required.
///
/// Adjust interrupt numbers and priorities based on your application.
pub fn init_nvic() {
	// interrupt::free ensures this setup runs with interrupts temporarily disabled (atomic operation).
    cortex_m::interrupt::free(|cs| {
		// NVIC::steal() gives us direct access to the NVIC registers (unsafe because it bypasses ownership checks).
        let mut nvic = unsafe { NVIC::steal() };

        // Example: Enable EXTI0 interrupt (IRQn = 6 on STM32F4, may differ on your MCU).
        unsafe {
			// nvic.enable(...) enables a specific interrupt (here, IRQ number 6, e.g., EXTI0 in STM32).
            nvic.enable(cortex_m::peripheral::Interrupt::from(6));
			// nvic.set_priority(..., 1) sets the interrupt priority (lower number = higher priority).
            nvic.set_priority(cortex_m::peripheral::Interrupt::from(6), 1);
        }

        // TODO: Add other interrupt configurations as needed
    });
}

/// Initialize the SysTick timer.
///
/// This function:
/// - Sets reload value for periodic interrupts.
/// - Selects clock source.
/// - Configures priority.
/// - Starts SysTick counter.
///
/// Adjust `tick_hz` and `core_hz` to your system.

/// Takes ownership of the SysTick peripheral (SYST).
/// Requires core_hz (CPU clock frequency) and 
/// tick_hz (desired SysTick frequency, e.g. 1000 for 1ms ticks).
pub fn init_systick(mut syst: SYST, core_hz: u32, tick_hz: u32) {
    // Example: 1ms tick (tick_hz = 1000) with system clock (core_hz).
    // SysTick is a 24-bit down-counter, so reload must fit in 24 bits.
	// Formula: if CPU = 48 MHz and you want 1 kHz ticks → reload = (48,000,000 / 1000) - 1 = 47,999.
	let reload = (core_hz / tick_hz) - 1;
    assert!(reload < (1 << 24), "SysTick reload value too large!");

	// Select clock source (Core = CPU clock).
    syst.set_clock_source(SystClkSource::Core);
	// Set reload value (when SysTick reaches 0, it reloads this value).
    syst.set_reload(reload);
	// Reset current counter value to start fresh.
    syst.clear_current();
	// Enable SysTick interrupts → triggers an interrupt handler every time counter reloads.
    syst.enable_interrupt();
	// Enable SysTick counter → actually starts the timer.
    syst.enable_counter();

    // Optionally set interrupt priority (lower number = higher priority)
    unsafe {
        let mut nvic = NVIC::steal();
		// SysTick also has a priority in the NVIC.
		// Here we set it to 2 (lower than EXTI0 = 1)
        nvic.set_priority(cortex_m::peripheral::Interrupt::SysTick, 2);
    }
}
