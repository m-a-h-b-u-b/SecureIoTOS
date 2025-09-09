//! SecureIoTOS net Module
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! Minimal, portable networking primitives and traits for SecureIoTOS.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(feature = "std")]
use std as core_std;

use core::fmt;

//! This module is intentionally small and dependency-light so it can be
//! integrated into embedded projects. It provides:
//! - `NetworkDevice` trait: low-level send/receive abstraction for a link
//! - `NetworkStack` struct: a tiny coordinator that can hold a device and
//!   perform simple operations (ARP/DHCP stubs left for integration)
//! - Small IP/address types and error handling
//! - Feature gates: `std` (enables std collections & tests) and `alloc`
//!
//! Guidance:
//! - For real embedded networking use `smoltcp`, `embassy-net`, or similar.
//!   This module is a thin, testable shim that lets higher-level code be
//!   written against an interface that can be adapted to those stacks.



/// Use alloc::vec::Vec when `alloc` feature is enabled; fall back to slice API otherwise.
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// IP address type (IPv4 only for now)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Addr {
    pub octets: [u8; 4],
}

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { octets: [a, b, c, d] }
    }

    pub const fn localhost() -> Self {
        Self::new(127, 0, 0, 1)
    }

    pub fn to_be_bytes(self) -> [u8; 4] {
        self.octets
    }
}

impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o = &self.octets;
        write!(f, "{}.{}.{}.{}", o[0], o[1], o[2], o[3])
    }
}

/// Simple network error enum used everywhere in this module.
#[derive(Debug)]
pub enum NetError {
    /// Device-specific IO error
    DeviceError,
    /// Packet was malformed or not supported
    MalformedPacket,
    /// Timeout waiting for an operation to complete
    Timeout,
    /// Operation not supported by the device/stack
    Unsupported,
    /// Generic failure with a textual message (requires `std` or `alloc`)
    #[cfg(any(feature = "alloc", feature = "std"))]
    Other(String),
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetError::DeviceError => write!(f, "device error"),
            NetError::MalformedPacket => write!(f, "malformed packet"),
            NetError::Timeout => write!(f, "timeout"),
            NetError::Unsupported => write!(f, "unsupported operation"),
            #[cfg(any(feature = "alloc", feature = "std"))]
            NetError::Other(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NetError {}

/// Result alias used throughout the network module.
pub type NetResult<T> = Result<T, NetError>;

/// A low-level network device abstraction. Implement this trait for your
/// hardware network interface (NIC, serial radio, etc.)
///
/// - `send` should transmit a raw layer-2 frame (Ethernet, 802.15.4, ...).
/// - `recv` should attempt to receive a raw frame into the provided buffer
///   and return the actual length on success. For non-blocking devices,
///   `Timeout` should be returned when no packet is available.
pub trait NetworkDevice {
    /// Transmit a buffer (frame) out via the device.
    fn send(&mut self, frame: &[u8]) -> NetResult<()>;

    /// Receive into the provided buffer, return number of bytes written.
    /// Non-blocking implementations can return `Err(NetError::Timeout)` when
    /// no data is available.
    fn recv(&mut self, buffer: &mut [u8]) -> NetResult<usize>;

    /// Optional: name or identifier for diagnostics
    fn name(&self) -> &str {
        "net-device"
    }

    /// Optional: MTU of the link (default 1500)
    fn mtu(&self) -> usize {
        1500
    }
}

/// Simple structure representing a bound interface (device + IP info)
pub struct NetInterface<D: NetworkDevice> {
    pub device: D,
    pub ip: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
}

impl<D: NetworkDevice> NetInterface<D> {
    /// Create a new interface from a device. IP config may be set later.
    pub fn new(device: D) -> Self {
        Self {
            device,
            ip: None,
            netmask: None,
            gateway: None,
        }
    }

    /// Configure static IPv4 address on the interface
    pub fn configure_ipv4(&mut self, ip: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr) {
        self.ip = Some(ip);
        self.netmask = Some(netmask);
        self.gateway = Some(gateway);
    }

    /// Send an IPv4 packet payload wrapped in a minimal IPv4 header.
    ///
    /// NOTE: This is a small helper to illustrate how the interface might be
    /// used; it produces a minimal IPv4 header (no options) and doesn't set
    /// all fields properly for production. Use a real IP stack in production.
    pub fn send_ipv4_payload(&mut self, dest: Ipv4Addr, payload: &[u8]) -> NetResult<()> {
        let src = self.ip.ok_or(NetError::Unsupported)?;
        let total_len = 20 + payload.len(); // IPv4 header (20) + payload
        if total_len > self.device.mtu() {
            return Err(NetError::MalformedPacket);
        }

        let mut frame: [u8; 1500] = [0u8; 1500];
        // IPv4 minimal header build (big-endian)
        // Version(4) + IHL(4)
        frame[0] = 0x45;
        // DSCP/ECN
        frame[1] = 0;
        // Total Length
        frame[2] = ((total_len >> 8) & 0xFF) as u8;
        frame[3] = (total_len & 0xFF) as u8;
        // Identification
        frame[4] = 0;
        frame[5] = 0;
        // Flags/Fragment offset
        frame[6] = 0;
        frame[7] = 0;
        // TTL
        frame[8] = 64;
        // Protocol: 0x11 = UDP (we're just illustrating)
        frame[9] = 0x11;
        // Header checksum (0 for now; a real stack would compute)
        frame[10] = 0;
        frame[11] = 0;
        // Src IP
        frame[12..16].copy_from_slice(&src.to_be_bytes());
        // Dst IP
        frame[16..20].copy_from_slice(&dest.to_be_bytes());
        // Payload
        let start = 20;
        frame[start..start + payload.len()].copy_from_slice(payload);

        self.device.send(&frame[..total_len])
    }

    /// Receive a raw frame (delegates to device)
    pub fn recv_frame(&mut self, buffer: &mut [u8]) -> NetResult<usize> {
        self.device.recv(buffer)
    }
}

/// Very small network stack wrapper which owns a single interface.
/// For real use you would expand this to support routing, ARP, DHCP, etc.
pub struct NetworkStack<D: NetworkDevice> {
    iface: NetInterface<D>,
}

impl<D: NetworkDevice> NetworkStack<D> {
    pub fn new(iface: NetInterface<D>) -> Self {
        Self { iface }
    }

    /// Configure static IPv4 address
    pub fn set_static_ipv4(&mut self, ip: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr) {
        self.iface.configure_ipv4(ip, netmask, gateway);
    }

    /// Send a small UDP-like payload to `dest`. This uses the `send_ipv4_payload`
    /// helper and sets "protocol" to UDP in the IPv4 header.
    pub fn send_udp_like(&mut self, dest: Ipv4Addr, payload: &[u8]) -> NetResult<()> {
        self.iface.send_ipv4_payload(dest, payload)
    }

    /// Poll for incoming frames and call the provided handler for each
    /// successfully received frame. The handler may return `false` to stop
    /// further processing.
    pub fn poll<F>(&mut self, mut handler: F) -> NetResult<()>
    where
        F: FnMut(&[u8]) -> bool,
    {
        let mut buf: [u8; 2048] = [0u8; 2048];
        match self.iface.recv_frame(&mut buf) {
            Ok(len) => {
                let cont = handler(&buf[..len]);
                if cont {
                    Ok(())
                } else {
                    Err(NetError::Unsupported) // signaling handler asked to stop
                }
            }
            Err(NetError::Timeout) => Err(NetError::Timeout),
            Err(e) => Err(e),
        }
    }
}

// ----------------------
// Minimal tests & examples
// ----------------------
#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// A tiny in-memory device useful for tests.
    struct LoopbackDevice {
        buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl LoopbackDevice {
        fn new() -> Self {
            Self {
                buffer: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl NetworkDevice for LoopbackDevice {
        fn send(&mut self, frame: &[u8]) -> NetResult<()> {
            let mut b = self.buffer.lock().unwrap();
            b.clear();
            b.extend_from_slice(frame);
            Ok(())
        }

        fn recv(&mut self, buffer: &mut [u8]) -> NetResult<usize> {
            let b = self.buffer.lock().unwrap();
            if b.is_empty() {
                return Err(NetError::Timeout);
            }
            let n = b.len().min(buffer.len());
            buffer[..n].copy_from_slice(&b[..n]);
            Ok(n)
        }

        fn name(&self) -> &str {
            "loopback"
        }

        fn mtu(&self) -> usize {
            1500
        }
    }

    #[test]
    fn test_send_and_recv_ipv4_payload() {
        let dev = LoopbackDevice::new();
        let mut iface = NetInterface::new(dev);
        iface.configure_ipv4(Ipv4Addr::new(10, 0, 0, 1), Ipv4Addr::new(255, 255, 255, 0), Ipv4Addr::new(10, 0, 0, 254));
        let mut stack = NetworkStack::new(iface);

        let payload = b"hello";
        let dest = Ipv4Addr::new(10, 0, 0, 2);
        // send
        stack.send_udp_like(dest, payload).expect("send failed");

        // poll and verify loopback received
        let res = stack.poll(|frame| {
            // Basic sanity: IPv4 header version/IHL
            assert_eq!(frame[0] >> 4, 4u8);
            true
        });

        assert!(res.is_ok());
    }
}
