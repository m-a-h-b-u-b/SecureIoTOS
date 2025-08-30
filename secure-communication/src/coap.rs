//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use coap_lite::{Packet, RequestType as Method};
use tokio::net::UdpSocket;

pub async fn send_demo() {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let addr = "127.0.0.1:5683";

    let mut request = Packet::new();
    request.set_method(Method::Get);
    request.set_path("/sensor/temp");

    let bytes = request.to_bytes().unwrap();
    socket.send_to(&bytes, addr).await.unwrap();

    println!("CoAP GET request sent to {}", addr);
}
