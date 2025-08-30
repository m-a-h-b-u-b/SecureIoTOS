//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use heapless::spsc::Queue;

static mut MSG_QUEUE: Queue<u32, 16> = Queue::new();

/// Initialize the message queue
pub fn init_queue() {
    unsafe { MSG_QUEUE.reset(); }
}

/// Send a message to the queue
pub fn send_message(msg: u32) {
    unsafe { MSG_QUEUE.enqueue(msg).ok(); }
}

/// Receive a message from the queue
pub fn receive_message() -> Option<u32> {
    unsafe { MSG_QUEUE.dequeue() }
}
