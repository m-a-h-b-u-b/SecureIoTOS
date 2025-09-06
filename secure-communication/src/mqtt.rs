//! SecureIoTOS MQTT Module
//! -----------------------
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Provides async MQTT client (TCP + TLS) for IoT devices
//! using the `rumqttc` crate.

use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS, Transport};
use std::time::Duration;
use anyhow::{Context, Result};
use tokio::time::sleep;

/// Create a new MQTT client (async) with TCP or TLS transport.
///
/// # Arguments
/// * `client_id` – Unique MQTT client ID
/// * `broker`    – MQTT broker hostname (e.g., "broker.hivemq.com")
/// * `port`      – Broker port (1883 for TCP, 8883 for TLS)
/// * `use_tls`   – If true, connect securely with TLS
pub fn mqtt_connect(client_id: &str, broker: &str, port: u16, use_tls: bool) -> (AsyncClient, EventLoop) {
    let mut mqttoptions = MqttOptions::new(client_id, broker, port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    if use_tls {
        mqttoptions.set_transport(Transport::Tls(rumqttc::TlsConfiguration::default()));
    }

    AsyncClient::new(mqttoptions, 10)
}

/// Publish a message to an MQTT topic.
pub async fn mqtt_publish(client: &AsyncClient, topic: &str, payload: &str) -> Result<()> {
    client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
        .with_context(|| format!("Failed to publish to topic {}", topic))?;

    println!(" Published to `{}`: {}", topic, payload);
    Ok(())
}

/// Subscribe to an MQTT topic.
pub async fn mqtt_subscribe(client: &AsyncClient, topic: &str) -> Result<()> {
    client
        .subscribe(topic, QoS::AtLeastOnce)
        .await
        .with_context(|| format!("Failed to subscribe to topic {}", topic))?;

    println!("📡 Subscribed to `{}`", topic);
    Ok(())
}

/// Run the MQTT event loop to process incoming messages and connection events.
pub async fn mqtt_event_loop(mut eventloop: EventLoop) -> Result<()> {
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                println!("Received on `{}`: {:?}", p.topic, String::from_utf8_lossy(&p.payload));
            }
            Ok(Event::Incoming(other)) => {
                println!("Incoming: {:?}", other);
            }
            Ok(Event::Outgoing(out)) => {
                println!("Outgoing: {:?}", out);
            }
            Err(e) => {
                eprintln!("MQTT error: {}", e);
                sleep(Duration::from_secs(3)).await;
                // TODO: add reconnect logic here if needed
            }
        }
    }
}
