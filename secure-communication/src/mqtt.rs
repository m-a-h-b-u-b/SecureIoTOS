//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;

pub async fn publish_demo() {
    let mut mqttoptions = MqttOptions::new("secure_iot_node", "broker.hivemq.com", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = Client::new(mqttoptions, 10);

    client.publish("secureiot/temperature", QoS::AtLeastOnce, false, "25 degree C").unwrap();
    println!("✅ MQTT message published");

    // Optional: simple event loop
    tokio::spawn(async move {
        loop {
            let event = eventloop.poll().unwrap();
            println!("MQTT Event: {:?}", event);
        }
    });
}
