//! SecureIoTOS IPC Module
//! ----------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/SecureIoTOS>
//!
//! Provides a single-producer, single-consumer (SPSC) message queue
//! using `heapless::spsc::Queue`. Designed for no_std and embedded environments.


use heapless::spsc::Queue;
use cortex_m::interrupt;

// Global SPSC message queue (u32, capacity 16)
static mut MSG_QUEUE: Queue<u32, 16> = Queue::new();

/// Initialize the message queue.
///
/// Resets the queue to empty. Should be called at system startup
/// before any send/receive operations.
pub fn init_queue() {
    interrupt::free(|_| unsafe { MSG_QUEUE.reset() });
}

//!
//! # Notes
//! - Capacity: 16 messages of type `u32`.
//! - `static mut` is used for global queue; access must be safe (interrupt-free or critical section).
//! - Optional: wrap send/receive in `Result` for overflow/empty detection in production.

/// Send a message to the queue.
///
/// # Arguments
/// * `msg` - u32 message to enqueue
///
/// Returns `Ok(())` if enqueued successfully, or `Err(())` if the queue is full.
pub fn send_message(msg: u32) -> Result<(), ()> {
    interrupt::free(|_| unsafe {
        MSG_QUEUE.enqueue(msg).map_err(|_| ())
    })
}

/// Receive a message from the queue.
///
/// Returns `Some(u32)` if a message was available, or `None` if the queue is empty.
pub fn receive_message() -> Option<u32> {
    interrupt::free(|_| unsafe { MSG_QUEUE.dequeue() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_send_receive() {
        init_queue();

        // Send messages
        assert_eq!(send_message(10), Ok(()));
        assert_eq!(send_message(20), Ok(()));

        // Receive messages
        assert_eq!(receive_message(), Some(10));
        assert_eq!(receive_message(), Some(20));
        assert_eq!(receive_message(), None);
    }
}
