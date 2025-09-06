//! SecureIoTOS CoAP Module
//! -----------------------
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
//!
//! Provides both a minimal async CoAP client and server for IoT devices.
//! Built on UDP + `coap-lite`.

use coap_lite::{Packet, RequestType as Method, ResponseType};
use tokio::net::UdpSocket;
use anyhow::{Context, Result};

/// ------------------ CLIENT ------------------

/// Generic CoAP request (used by GET/POST/PUT/DELETE).
pub async fn coap_request(
    addr: &str,
    method: Method,
    path: &str,
    payload: Option<&[u8]>,
) -> Result<Packet> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .context("Failed to bind UDP socket")?;

    let mut request = Packet::new();
    request.set_method(method);
    request.set_path(path);
    if let Some(data) = payload {
        request.payload = data.to_vec();
    }

    let req_bytes = request.to_bytes()
        .context("Failed to serialize CoAP request")?;

    socket.send_to(&req_bytes, addr)
        .await
        .with_context(|| format!("Failed to send CoAP request to {}", addr))?;

    let mut buf = [0u8; 1500]; // UDP MTU
    let (size, _) = socket.recv_from(&mut buf)
        .await
        .context("Failed to receive CoAP response")?;

    let response = Packet::from_bytes(&buf[..size])
        .context("Failed to parse CoAP response")?;

    Ok(response)
}

pub async fn coap_get(addr: &str, path: &str) -> Result<Packet> {
    coap_request(addr, Method::Get, path, None).await
}

pub async fn coap_post(addr: &str, path: &str, payload: &[u8]) -> Result<Packet> {
    coap_request(addr, Method::Post, path, Some(payload)).await
}

pub async fn coap_put(addr: &str, path: &str, payload: &[u8]) -> Result<Packet> {
    coap_request(addr, Method::Put, path, Some(payload)).await
}

pub async fn coap_delete(addr: &str, path: &str) -> Result<Packet> {
    coap_request(addr, Method::Delete, path, None).await
}

/// ------------------ SERVER ------------------

/// Minimal async CoAP server.
/// 
/// # Arguments
/// * `bind_addr` – UDP socket to bind (e.g., "0.0.0.0:5683")
///
/// The server listens forever and responds with simple demo payloads.
pub async fn coap_server(bind_addr: &str) -> Result<()> {
    let socket = UdpSocket::bind(bind_addr)
        .await
        .with_context(|| format!("Failed to bind CoAP server on {}", bind_addr))?;

    println!("CoAP server listening on {}", bind_addr);

    let mut buf = [0u8; 1500];

    loop {
        let (size, peer) = socket.recv_from(&mut buf)
            .await
            .context("Failed to receive CoAP request")?;

        if let Ok(request) = Packet::from_bytes(&buf[..size]) {
            let mut response = Packet::new();
            response.header.message_id = request.header.message_id;
            response.set_token(request.get_token().clone());

            match request.get_method() {
                Some(Method::Get) => {
                    if request.get_path() == "/sensor/temp" {
                        response.header.code = ResponseType::Content.into();
                        response.payload = b"23.5°C".to_vec();
                    } else {
                        response.header.code = ResponseType::NotFound.into();
                    }
                }
                Some(Method::Post) => {
                    response.header.code = ResponseType::Created.into();
                    response.payload = request.payload.clone(); // echo back
                }
                Some(Method::Put) => {
                    response.header.code = ResponseType::Changed.into();
                    response.payload = request.payload.clone();
                }
                Some(Method::Delete) => {
                    response.header.code = ResponseType::Deleted.into();
                }
                _ => {
                    response.header.code = ResponseType::MethodNotAllowed.into();
                }
            }

            if let Ok(res_bytes) = response.to_bytes() {
                socket.send_to(&res_bytes, peer).await?;
            }
        }
    }
}

/// ------------------ TESTS ------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task;

    #[tokio::test]
    async fn test_coap_client_server() {
        // Run server in background
        task::spawn(async {
            coap_server("127.0.0.1:5683").await.unwrap();
        });

        // Give server a moment to start
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Test GET
        let res = coap_get("127.0.0.1:5683", "/sensor/temp").await.unwrap();
        assert_eq!(String::from_utf8_lossy(&res.payload), "23.5°C");

        // Test POST
        let res = coap_post("127.0.0.1:5683", "/sensor/data", b"42").await.unwrap();
        assert_eq!(res.payload, b"42");
    }
}
