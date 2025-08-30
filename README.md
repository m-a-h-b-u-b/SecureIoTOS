# SecureIoTOS 🛡️
A Rust-based Secure Tiny OS for ARM Cortex-M IoT Devices.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Overview
SecureIoTOS is a bare-metal operating system for ARM Cortex-M microcontrollers, built in Rust.  
It demonstrates secure boot, memory protection, process isolation, safe drivers, and secure communication for IoT devices.

This repository provides all source code and examples to build, run, and test the OS on both QEMU and real IoT boards.

## Repository Structure

```
SecureIoTOS/
├── bootloader/       # Secure bootloader with firmware verification
├── kernel/           # Core kernel (scheduler, syscalls, MPU, NVIC)
├── memory/           # Memory management and Rust-safe abstractions
├── ipc/              # Task scheduling and inter-process communication
├── hal/              # Hardware Abstraction Layer (GPIO, UART, SPI, I2C, timers)
├── drivers/          # Safe, interrupt-driven device drivers
├── crypto/           # Cryptography modules (AES, ECC, RNG)
├── net/              # Secure communication (TLS/DTLS, MQTT, CoAP)
├── examples/         # Sample IoT applications
├── tests/            # Security and unit tests
└── tools/            # Build, flash, and emulation scripts
```

## Getting Started

### Prerequisites
- Rust nightly (`rustup default nightly`)
- ARM target:  
  ```bash
  rustup target add thumbv7em-none-eabi
  ```
- QEMU (optional) or IoT development board

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
Use the scripts in `/tools` to flash binaries to your board.

## Features
- Secure bootloader with firmware verification  
- MPU-based memory protection & process isolation  
- Preemptive and cooperative scheduler  
- Hardware abstraction and secure device drivers  
- Secure cryptography and communication stack  
- Examples demonstrating real IoT applications  

## License
Apache 2.0 — see [LICENSE](LICENSE).

## Author
Md Mahbubur Rahman  
[GitHub](https://github.com/m-a-h-b-u-b) | [Website](https://m-a-h-b-u-b.github.io)
