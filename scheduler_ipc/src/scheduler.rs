//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::tasks::Task;
use crate::tasks::context_switch;

/// Round-robin scheduler for tasks
pub struct Scheduler {
    tasks: Vec<Task>,
    current: usize,
}

impl Scheduler {
    pub fn new(tasks: Vec<Task>) -> Self {
        Scheduler { tasks, current: 0 }
    }

    pub fn schedule(&mut self) {
        let next = (self.current + 1) % self.tasks.len();
        context_switch(&self.tasks[self.current], &self.tasks[next]);
        self.current = next;
    }
}
