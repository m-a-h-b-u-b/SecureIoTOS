//! SecureIoTOS Kernel: Production-ready Syscall Layer
//! -------------------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author : Md Mahbubur Rahman
//! URL    : https://m-a-h-b-u-b.github.io
//! GitHub : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Features:
//! - Multi-argument support (up to 6 args) typical for modern ABIs.
//! - Handlers return `Result<u32, SyscallError>`; the C entrypoint encodes this
//!   into a `u32` return value (success = value, error = high-bit set + code).
//! - Thin capability/privilege check example.
//! - Secure user-memory copy helpers (stubs; MUST be implemented per-arch).
//! - Trait-based handlers for composability and unit testing.

#![no_std] // comment out if you want std during testing
#![allow(dead_code)]

use core::convert::TryFrom;

/// Maximum syscall arguments we'll support here (adjust for target ABI).
pub const MAX_SYSCALL_ARGS: usize = 6;

/// Error codes (repr u32 so they can be encoded into raw return).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallError {
    Invalid = 1,
    BadAddress = 2,
    PermissionDenied = 3,
    TooLarge = 4,
    NotFound = 5,
    Unsupported = 6,
    Unknown = 0xFFFF,
}

impl From<SyscallError> for u32 {
    fn from(e: SyscallError) -> u32 {
        e as u32
    }
}

/// A raw-user-visible encoding scheme for errors:
/// - If top bit (0x8000_0000) is 0 => success, value is the return value.
/// - If top bit is 1 => failure, lower 31 bits contain error code (SyscallError as u32).
#[inline]
fn encode_syscall_result(res: Result<u32, SyscallError>) -> u32 {
    match res {
        Ok(v) => v,
        Err(e) => 0x8000_0000u32 | (u32::from(e) & 0x7FFF_FFFF),
    }
}

/// Representation for up to 6 syscall args (common in many ABIs).
#[derive(Debug, Clone, Copy)]
pub struct SyscallArgs {
    pub args: [u64; MAX_SYSCALL_ARGS], // using u64 to fit 64-bit ABIs; truncate when needed
    pub nargs: usize,
}

impl SyscallArgs {
    pub fn arg_u32(&self, idx: usize) -> Result<u32, SyscallError> {
        if idx >= self.nargs { return Err(SyscallError::Invalid); }
        Ok(self.args[idx] as u32)
    }

    pub fn arg_u64(&self, idx: usize) -> Result<u64, SyscallError> {
        if idx >= self.nargs { return Err(SyscallError::Invalid); }
        Ok(self.args[idx])
    }
}

/// Syscall identifiers. Keep stable values for ABI compatibility.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallId {
    GetTime = 1,
    SendMessage = 2,
    // add more here...
}

impl TryFrom<u32> for SyscallId {
    type Error = ();
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(SyscallId::GetTime),
            2 => Ok(SyscallId::SendMessage),
            _ => Err(()),
        }
    }
}

/// Represents the current execution context (thread/process) — minimal stub.
/// Fill with real fields in your kernel (UID/GID, capabilities, address space, etc).
#[derive(Debug)]
pub struct CurrentContext {
    pub uid: u32,
    pub capabilities: u32,
}

/// Example capability flags (you can use bitflags crate in std builds).
pub mod caps {
    pub const SYS_TIME: u32 = 1 << 0;
    pub const SEND_MESSAGE: u32 = 1 << 1;
}

/// Return the current execution context (stub — implement per-kernel).
fn current_context() -> CurrentContext {
    // TODO: get context from scheduler / current thread struct
    CurrentContext {
        uid: 0,
        capabilities: caps::SYS_TIME | caps::SEND_MESSAGE,
    }
}

/// Trait for syscall handlers. Implementations return `Result<u32, SyscallError>`.
pub trait SyscallHandler {
    fn handle(&self, ctx: &CurrentContext, args: &SyscallArgs) -> Result<u32, SyscallError>;
}

/// Syscall dispatcher: maps `SyscallId` -> handler object.
pub fn dispatch_syscall(id: SyscallId, ctx: &CurrentContext, args: &SyscallArgs) -> Result<u32, SyscallError> {
    match id {
        SyscallId::GetTime => GetTimeSyscall.handle(ctx, args),
        SyscallId::SendMessage => SendMessageSyscall.handle(ctx, args),
    }
}

/// -----------------
/// Implementations
/// -----------------

/// GetTime Syscall: returns a 32-bit time value (seconds since epoch or ticks).
pub struct GetTimeSyscall;

impl SyscallHandler for GetTimeSyscall {
    fn handle(&self, ctx: &CurrentContext, _args: &SyscallArgs) -> Result<u32, SyscallError> {
        // Privilege check: ensure caller has SYS_TIME capability.
        if (ctx.capabilities & caps::SYS_TIME) == 0 {
            return Err(SyscallError::PermissionDenied);
        }

        // TODO: Replace with real RTC/clock reading
        let secs: u32 = kernel_get_time_seconds();
        Ok(secs)
    }
}

/// SendMessage Syscall:
/// Args:
/// - arg0: user-space pointer (u32/usize) to buffer
/// - arg1: length (u32)
/// - arg2: destination id (u32)
pub struct SendMessageSyscall;

impl SyscallHandler for SendMessageSyscall {
    fn handle(&self, ctx: &CurrentContext, args: &SyscallArgs) -> Result<u32, SyscallError> {
        if (ctx.capabilities & caps::SEND_MESSAGE) == 0 {
            return Err(SyscallError::PermissionDenied);
        }

        let ptr = args.arg_u64(0).map_err(|_| SyscallError::Invalid)? as usize;
        let len = args.arg_u64(1).map_err(|_| SyscallError::Invalid)? as usize;
        let dest = args.arg_u64(2).map_err(|_| SyscallError::Invalid)? as u32;

        if len == 0 || len > 4096 {
            return Err(SyscallError::TooLarge);
        }

        // Copy data from user space into kernel-owned buffer (safe copy).
        let mut buf = vec![0u8; len]; // NOTE: replace with kernel allocator if no std
        copy_from_user(ptr, &mut buf).map_err(|_| SyscallError::BadAddress)?;

        // TODO: enqueue message into IPC subsystem
        kernel_ipc_send(dest, &buf).map_err(|_| SyscallError::Unknown)?;

        Ok(0) // success, return 0
    }
}

/// ---------------
/// Kernel primitives (stubs - platform-specific)
/// ---------------

/// Example kernel time source (stub). Replace with RTC or clocksource.
fn kernel_get_time_seconds() -> u32 {
    // TODO: integrate with platform clocksource
    1_700_000_000u32 // placeholder epoch-like value
}

/// IPC sending primitive (stub). Return Ok(()) on success.
fn kernel_ipc_send(_dest: u32, _buf: &[u8]) -> Result<(), ()> {
    // TODO: implement message queue / sockets / ports
    Ok(())
}

/// Securely copy memory from user address space into a kernel buffer.
/// IMPORTANT: This is architecture- and MMU-specific. This stub MUST be replaced
/// by a function that:
///  - validates the user pointer is mapped and accessible,
///  - disables preemption if necessary,
///  - uses safe primitives for copy (e.g., copy_from_user on Linux),
///  - returns Err on any memory fault or invalid mapping.
fn copy_from_user(user_ptr: usize, dst: &mut [u8]) -> Result<(), ()> {
    // STUB: in a real kernel this must not just do pointer casts.
    // Replace with: verify user mapping, then memcopy with fault handling.
    unsafe {
        let user_slice = core::slice::from_raw_parts(user_ptr as *const u8, dst.len());
        dst.copy_from_slice(user_slice);
    }
    Ok(())
}

/// Validate a user pointer / length (example stub).
fn validate_user_ptr(_ptr: usize, _len: usize) -> bool {
    // TODO: check that [ptr, ptr+len) is in user-space portion and mapped
    true
}

/// -----------------
/// C ABI syscall entrypoint
/// -----------------
/// This function has `extern "C"` and no_mangle so the architecture's syscall
/// trampoline can call it directly. It receives an id and up to 6 register
/// arguments (u64 each). Adapt arg types to match your platform (u32/u64).
///
/// The entrypoint:
/// - constructs `SyscallArgs`,
/// - resolves `SyscallId`,
/// - obtains `CurrentContext`,
/// - calls dispatcher,
/// - returns encoded `u32` result (success or error-encoded).
///
/// NOTE: On 64-bit platforms the architecture will usually pass arguments in
/// registers (e.g., rdi/rsi/...); ensure your syscall trampoline forwards them.

#[no_mangle]
pub extern "C" fn syscall_entry(
    raw_id: u32,
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
) -> u32 {
    // Build args structure
    let all_args = [a0, a1, a2, a3, a4, a5];
    // In many ABIs the number of args isn't passed; we assume maximum and handlers check
    let args = SyscallArgs { args: all_args, nargs: MAX_SYSCALL_ARGS };

    // Resolve syscall id
    let id = match SyscallId::try_from(raw_id) {
        Ok(id) => id,
        Err(_) => return encode_syscall_result(Err(SyscallError::Invalid)),
    };

    // Fetch current context (implement per-kernel)
    let ctx = current_context();

    // Dispatch
    let res = dispatch_syscall(id, &ctx, &args);

    // Encode result for userland
    encode_syscall_result(res)
}

/// -----------------
/// Testing helpers (only when std is available)
/// -----------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_errors() {
        let ok = encode_syscall_result(Ok(42));
        assert_eq!(ok, 42);

        let err = encode_syscall_result(Err(SyscallError::PermissionDenied));
        assert_eq!(err & 0x8000_0000, 0x8000_0000);
        assert_eq!(err & 0x7FFF_FFFF, u32::from(SyscallError::PermissionDenied));
    }

    #[test]
    fn get_time_via_dispatch() {
        let ctx = CurrentContext { uid: 0, capabilities: caps::SYS_TIME };
        let args = SyscallArgs { args: [0; MAX_SYSCALL_ARGS], nargs: 0 };
        let r = dispatch_syscall(SyscallId::GetTime, &ctx, &args);
        assert!(r.is_ok());
    }
}
