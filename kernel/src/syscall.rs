//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

#[no_mangle]
pub extern "C" fn syscall_handler(call_id: u32, arg: u32) -> u32 {
    match call_id {
        1 => get_time(),
        2 => send_message(arg),
        _ => 0,
    }
}

fn get_time() -> u32 { 0 }
fn send_message(_arg: u32) -> u32 { 1 }
