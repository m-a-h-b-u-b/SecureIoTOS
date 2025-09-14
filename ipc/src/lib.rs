//! SecureIoTOS IPC Library Module
//! ---------------------------------
//! License : Dual License
//!   - Apache 2.0 for open-source / personal use
//!   - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

//! This module provides IPC primitives — mechanisms for 
//! tasks (threads, processes, or lightweight tasks in SecureIoTOS) to communicate and synchronize safely:
//! Message Queues (for passing data between tasks)
//! Semaphores (for signaling between tasks)
//! Event Flags (for task synchronization via event triggers)

// #![no_std]: disables Rust’s standard library, ensuring compatibility with embedded systems.
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// UnsafeCell: allows mutable memory inside immutable structs, 
// needed for concurrency (e.g., message queue head/tail).
use core::cell::UnsafeCell;

// AtomicBool: provides lock-free synchronization for semaphores/events
// Ordering: defines memory ordering guarantees (Acquire, Release, etc.).
use core::sync::atomic::{AtomicBool, Ordering};

// A generic fixed-size message container (N = max message size).
// Example: IpcMessage<16> → holds up to 16 bytes.
// length specifies how many bytes are actually used.
#[derive(Debug, Clone, Copy)]
pub struct IpcMessage<const N: usize> {
    pub data: [u8; N],
    pub length: usize,
}

impl<const N: usize> IpcMessage<N> {
    pub const fn new() -> Self {
        Self {
            data: [0u8; N],
            length: 0,
        }
    }
}

/// Simple single-producer, single-consumer message queue.
/// Can be used for task-to-task communication.
/// SIZE = number of messages it can store.
/// MSG_SIZE = max size of each message.
/// Uses a circular buffer with head (enqueue index) and tail (dequeue index).
pub struct MessageQueue<const SIZE: usize, const MSG_SIZE: usize> {
    buffer: [IpcMessage<MSG_SIZE>; SIZE],
    head: UnsafeCell<usize>,
    tail: UnsafeCell<usize>,
}

impl<const SIZE: usize, const MSG_SIZE: usize> MessageQueue<SIZE, MSG_SIZE> {
    /// Creates a new empty queue
    pub const fn new() -> Self {
        const EMPTY: IpcMessage<0> = IpcMessage { data: [], length: 0 };
        // SAFETY: casting array of zero-sized to MSG_SIZE
        let buffer: [IpcMessage<MSG_SIZE>; SIZE] = unsafe { core::mem::transmute([EMPTY; SIZE]) };
        Self {
            buffer,
            head: UnsafeCell::new(0),
            tail: UnsafeCell::new(0),
        }
    }

    /// Enqueue a message
    pub fn enqueue(&self, msg: IpcMessage<MSG_SIZE>) -> Result<(), ()> {
        let head = unsafe { *self.head.get() };
        let next_head = (head + 1) % SIZE;
        let tail = unsafe { *self.tail.get() };

        if next_head == tail {
            return Err(()); // Queue full
        }

        self.buffer[head] = msg;
        unsafe { *self.head.get() = next_head };
        Ok(())
    }

    /// Dequeue a message
    pub fn dequeue(&self) -> Option<IpcMessage<MSG_SIZE>> {
        let tail = unsafe { *self.tail.get() };
        let head = unsafe { *self.head.get() };

        if tail == head {
            return None; // Queue empty
        }

        let msg = self.buffer[tail];
        unsafe { *self.tail.get() = (tail + 1) % SIZE };
        Some(msg)
    }
}

/// Simple binary semaphore for signaling between tasks.
pub struct Semaphore {
    flag: AtomicBool,
}

impl Semaphore {
    pub const fn new(initial: bool) -> Self {
        Self {
            flag: AtomicBool::new(initial),
        }
    }

    /// Signal / release the semaphore
    pub fn signal(&self) {
        self.flag.store(true, Ordering::Release);
    }

    /// Wait for the semaphore. Returns true if acquired, false if not set.
    pub fn wait(&self) -> bool {
        self.flag.swap(false, Ordering::AcqRel)
    }

    /// Check if the semaphore is currently set
    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }
}

/// Event flags structure (32-bit flags)
pub struct EventFlags {
    flags: AtomicBool,
}

impl EventFlags {
    pub const fn new() -> Self {
        Self {
            flags: AtomicBool::new(false),
        }
    }

    /// Set event flag
    pub fn set(&self) {
        self.flags.store(true, Ordering::Release);
    }

    /// Clear event flag
    pub fn clear(&self) {
        self.flags.store(false, Ordering::Release);
    }

    /// Wait for event flag
    pub fn wait(&self) -> bool {
        self.flags.swap(false, Ordering::AcqRel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_queue() {
        const SIZE: usize = 4;
        const MSG_SIZE: usize = 8;
        let queue: MessageQueue<SIZE, MSG_SIZE> = MessageQueue::new();

        let msg = IpcMessage {
            data: [1, 2, 3, 4, 5, 6, 7, 8],
            length: 8,
        };

        queue.enqueue(msg).unwrap();
        let received = queue.dequeue().unwrap();
        assert_eq!(received.data[0], 1);
    }

    #[test]
    fn test_semaphore() {
        let sem = Semaphore::new(false);
        assert!(!sem.wait());
        sem.signal();
        assert!(sem.wait());
        assert!(!sem.wait());
    }

    #[test]
    fn test_event_flags() {
        let evt = EventFlags::new();
        assert!(!evt.wait());
        evt.set();
        assert!(evt.wait());
        assert!(!evt.wait());
    }
}
