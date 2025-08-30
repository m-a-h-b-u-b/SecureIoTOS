//! SecureIoTOS Kernel Module
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use tokio_rustls::TlsConnector;
use tokio::net::TcpStream;
use std::sync::Arc;
use webpki_roots::TLS_SERVER_ROOTS;

pub async fn connect_tls(addr: &str) {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject, ta.spki, ta.name_constraints
        )
    }));

    let config = Arc::new(ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth());

    let connector = TlsConnector::from(config);
    match TcpStream::connect(addr).await {
        Ok(stream) => {
            let domain = rustls::ServerName::try_from("example.com").unwrap();
            match connector.connect(domain, stream).await {
                Ok(_) => println!("TLS connection established to {}", addr),
                Err(e) => println!(" TLS connection failed: {}", e),
            }
        }
        Err(e) => println!("❌ TCP connection failed: {}", e),
    }
}
