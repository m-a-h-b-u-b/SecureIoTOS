//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::context::context_switch;
use crate::init::get_tasks;
use crate::context::Task;

pub fn schedule() {
    let current = get_tasks()[0].clone();
    let next = get_tasks()[1].clone();
    context_switch(&current, &next);
}
