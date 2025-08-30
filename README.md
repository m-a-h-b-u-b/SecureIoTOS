# SecureIoTOS 🛡️

**A Rust-based Secure Tiny OS for ARM Cortex-M IoT Devices**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Build](https://github.com/<user>/SecureIoTOS/actions/workflows/build.yml/badge.svg)](https://github.com/<user>/SecureIoTOS/actions)

---

## Overview

SecureIoTOS is a **bare-metal operating system for ARM Cortex-M microcontrollers**, written in Rust.
It demonstrates **secure boot, memory protection, process isolation, safe drivers, cryptography, and secure communication** for IoT devices.

The repository provides fully modular source code and examples to **build, run, and test the OS** on both QEMU and real IoT boards like STM32, ESP32, and NRF52.

---

## Features

* Secure bootloader with SHA256/RSA firmware verification
* MPU-based memory protection and process isolation
* Preemptive and cooperative task scheduler
* Hardware abstraction for GPIO, UART, SPI, I2C, timers
* Safe, interrupt-driven device drivers
* Secure cryptography modules: AES, ECC, RNG
* TLS/DTLS and lightweight IoT messaging protocols: MQTT, CoAP
* Example IoT applications: sensor nodes, telemetry, “Hello World”

---

## Core Module Flow

```plaintext
                +----------------+
                |  bootloader/   |
                | Secure Boot &  |
                | Firmware Verify|
                +-------+--------+
                        |
                        v
                +----------------+
                |    kernel/     |
                | Scheduler,     |
                | Syscalls, MPU  |
                +-------+--------+
                        |
        +---------------+----------------+
        |                                |
        v                                v
  +------------+                    +-----------+
  |   memory/  |                    |   ipc/   |
  | Memory Mgmt|                    | Messaging|
  | & Rust-safe|                    | Channels |
  | Abstractions|                   +-----------+
        |
        v
  +------------+
  |   hal/     |
  | GPIO, UART,|
  | SPI, I2C   |
  +------------+
        |
        v
  +------------+
  |  drivers/  |
  | Device     |
  | Drivers    |
  +------------+
        |
        v
  +------------+       +-----------+
  |  crypto/   |       |   net/    |
  | AES, ECC,  |       | TLS/DTLS, |
  | RNG        |       | MQTT, CoAP|
  +------------+       +-----------+
        |
        v
  +------------+
  | examples/  |
  | IoT Apps   |
  +------------+
```

---

## Repository Structure

| Folder     | Purpose                                                   |
| ---------- | --------------------------------------------------------- |
| bootloader | Secure bootloader and firmware verification               |
| kernel     | Core kernel: scheduler, syscalls, MPU handling            |
| memory     | Memory management and Rust-safe abstractions              |
| ipc        | Task communication primitives                             |
| hal        | MCU peripheral abstraction (GPIO, UART, SPI, I2C, timers) |
| drivers    | Safe, interrupt-driven device drivers                     |
| crypto     | Cryptography modules (AES, ECC, RNG)                      |
| net        | TLS/DTLS, MQTT, CoAP for secure communication             |
| examples   | Sample IoT applications                                   |
| tests      | Security and unit tests                                   |
| tools      | Build, flash, and QEMU scripts                            |

---

## Getting Started

### Prerequisites

* Rust nightly (`rustup default nightly`)
* ARM target:

```bash
rustup target add thumbv7em-none-eabi
```

* QEMU (optional) or IoT development board (STM32, ESP32, NRF52)

### Build Example

```bash
cd bootloader
cargo build --target thumbv7em-none-eabi
```

### Run on QEMU

```bash
qemu-system-arm -M stm32-p103 -kernel target/thumbv7em-none-eabi/debug/bootloader
```

### Run on Hardware

Use the scripts in `/tools` to flash binaries to your board:

```bash
./tools/flash.sh <board>
```

---

## Examples

* `examples/hello_world/` → Minimal Rust “Hello World” app
* `examples/sensor_node/` → Reads sensor data and prints via UART
* `examples/telemetry/` → Secure telemetry system sending data over MQTT/DTLS

---

## License

Apache 2.0 — see [LICENSE](LICENSE)

---

## Author

**Md Mahbubur Rahman**
[GitHub](https://github.com/m-a-h-b-u-b) | [Website](https://m-a-h-b-u-b.github.io)

---

## Contributing

We welcome contributions!

* Fork the repo and submit pull requests
* Follow Rust coding guidelines and safety best practices
* Report issues or suggest features via GitHub Issues

---

## References & Further Reading

* [Rust Embedded Book](https://docs.rust-embedded.org/book/)
* [ARM Cortex-M Technical Reference Manual](https://developer.arm.com/documentation)
* [Embedded Systems Security Principles](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-183.pdf)
