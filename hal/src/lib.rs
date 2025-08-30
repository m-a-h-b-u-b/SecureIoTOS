//! SecureIoTOS HAL Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod gpio;
pub mod timer;
pub mod bus;

/// Initialize HAL modules
pub fn init_hal() {
    gpio::init_gpio();
    timer::init_timer();
    bus::init_bus();
}