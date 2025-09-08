//! SecureIoTOS IPC Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use heapless::spsc::Queue;

// Global single-producer, single-consumer message queue.
// Capacity: 16 messages, each of type `u32`.
// Declared `static mut` for use in a no_std environment, but requires
// careful handling since it is not inherently thread-safe.
static mut MSG_QUEUE: Queue<u32, 16> = Queue::new();

/// Initialize the message queue.
///
/// This resets the queue to an empty state.
/// Should be called during system startup before tasks attempt to
/// send or receive messages.
pub fn init_queue() {
    unsafe { MSG_QUEUE.reset(); }
}

/// Send a message to the queue.
///
/// Attempts to enqueue a `u32` message into the global queue.
/// If the queue is full, the message is silently dropped
/// (`ok()` ignores the error).
///
/// In a production system, it may be preferable to return a `Result`
/// so the caller can detect and handle queue overflow.
pub fn send_message(msg: u32) {
    unsafe { MSG_QUEUE.enqueue(msg).ok(); }
}

/// Receive a message from the queue.
///
/// Attempts to dequeue a message from the global queue.
/// Returns:
/// - `Some(u32)` if a message was available,
/// - `None` if the queue was empty.
pub fn receive_message() -> Option<u32> {
    unsafe { MSG_QUEUE.dequeue() }
}
