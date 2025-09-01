//! SecureIoTOS Scheduler Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use crate::tasks::Task;
use crate::tasks::context_switch;

/// A simple round-robin task scheduler.
///
/// The scheduler cycles through a list of tasks in order, giving each
/// task a chance to run. After reaching the last task, it wraps around
/// to the first one again. This approach ensures fairness but does not
/// consider task priority or deadlines.
pub struct Scheduler {
    /// List of all tasks managed by the scheduler.
    tasks: Vec<Task>,
    /// Index of the currently running task.
    current: usize,
}

impl Scheduler {
    /// Create a new scheduler with a given list of tasks.
    ///
    /// The scheduler starts with `current` set to 0, meaning the first
    /// task in the list will run initially.
    pub fn new(tasks: Vec<Task>) -> Self {
        Scheduler { tasks, current: 0 }
    }

    /// Perform a scheduling step.
    ///
    /// - Determines the next task index in round-robin order.
    /// - Performs a context switch from the current task to the next task.
    /// - Updates the `current` index to point to the task that is now running.
    pub fn schedule(&mut self) {
        let next = (self.current + 1) % self.tasks.len();
        context_switch(&self.tasks[self.current], &self.tasks[next]);
        self.current = next;
    }
}
