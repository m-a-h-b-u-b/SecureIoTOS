//! SecureIoTOS Telemetry Module
//! ---------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! This module defines a telemetry system for collecting and securely
//! transmitting sensor data in IoT devices.

use crate::sensor;
use serde::{Serialize, Deserialize};
use log::{info, error};

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

/// Telemetry data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryData {
    pub temperature: f32,
    pub humidity: f32,
}

/// Trait for all telemetry sources (extensible for more sensors)
pub trait TelemetrySource {
    fn read(&self) -> Result<f32, &'static str>;
}

/// Example: Temperature sensor implementation
pub struct TemperatureSensor;

impl TelemetrySource for TemperatureSensor {
    fn read(&self) -> Result<f32, &'static str> {
        sensor::read_sensor().ok_or("Temperature sensor read failed")
    }
}

/// Example: Humidity sensor (simulated)
pub struct HumiditySensor;

impl TelemetrySource for HumiditySensor {
    fn read(&self) -> Result<f32, &'static str> {
        Ok(50.0) // Placeholder simulation
    }
}

/// Collect telemetry data from multiple sensors
pub fn collect_telemetry() -> Result<TelemetryData, &'static str> {
    let temp_sensor = TemperatureSensor;
    let humidity_sensor = HumiditySensor;

    Ok(TelemetryData {
        temperature: temp_sensor.read()?,
        humidity: humidity_sensor.read()?,
    })
}

/// Securely transmit telemetry data:
/// 1. Serialize to JSON
/// 2. Encrypt with AES-256-GCM
/// 3. Base64-encode and (for demo) log the payload
///
/// `key_bytes` must be a 32-byte symmetric key managed securely
pub fn transmit_telemetry(
    data: &TelemetryData,
    key_bytes: &[u8; 32],
) -> Result<(), &'static str> {
    // --- 1. Serialize ---
    let json_payload = serde_json::to_string(data)
        .map_err(|_| {
            error!("Telemetry serialization failed");
            "Serialization error"
        })?;

    // --- 2. Encrypt ---
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // AES-GCM requires a unique 96-bit (12-byte) nonce per message
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, json_payload.as_bytes())
        .map_err(|_| {
            error!("Telemetry encryption failed");
            "Encryption error"
        })?;

    // Prepend nonce so receiver can decrypt
    let mut message = nonce_bytes.to_vec();
    message.extend_from_slice(&ciphertext);

    // --- 3. Encode & "send" ---
    let encoded = general_purpose::STANDARD.encode(message);

    // In production: send `encoded` via HTTPS/MQTT/etc.
    info!("Securely transmitting telemetry payload: {}", encoded);

    Ok(())
}
