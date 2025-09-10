//! SecureIoTOS TLS Module
//! -----------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author	: Md Mahbubur Rahman
//! URL		: https://m-a-h-b-u-b.github.io
//! GitHub	: https://github.com/m-a-h-b-u-b/SecureIoTOS
//! 
//! This module provides a minimal async TLS client connector
//! built on top of tokio-rustls with system root certificates.

use tokio_rustls::rustls::{
    ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName,
};
use tokio_rustls::{TlsConnector, client::TlsStream};
use tokio::net::TcpStream;
use std::sync::Arc;
use webpki_roots::TLS_SERVER_ROOTS;
use anyhow::{Context, Result};

/// Establish a secure TLS connection to the given address and domain.
///
/// # Arguments
/// * `addr`   – The remote socket address (e.g., "93.184.216.34:443")
/// * `domain` – The expected TLS server name (e.g., "example.com")
///
/// # Returns
/// A `TlsStream<TcpStream>` if successful.
///
/// # Errors
/// Returns an error if TCP or TLS handshake fails.
pub async fn connect_tls(addr: &str, domain: &str) -> Result<TlsStream<TcpStream>> {
    // Build root certificate store
	// Creates an empty certificate store.		
	// This store will later hold trusted root certificates (Certificate Authorities, or CAs).
	// A TLS client uses this store to check if the server’s certificate is signed by a trusted CA.
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(
		// Here, .0 extracts the internal slice of trust anchors.
        TLS_SERVER_ROOTS.0.iter().map(|ta| {
			// For each trust anchor (ta) from Mozilla’s list, it creates an OwnedTrustAnchor.
			// A TrustAnchor is basically:	
			// subject → who issued the certificate (the CA).
			// spki → the public key info.
			// name_constraints → restrictions on which domains the cert can issue for.
			
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject, ta.spki, ta.name_constraints,
            )
        })
    );

    // Configure TLS client
	// wraps config in an atomic reference counter, since it may be shared across tasks/streams.
    let config = Arc::new(
        ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    );
	// creates a connector object that can perform TLS handshakes.
    let connector = TlsConnector::from(config);

    // Establish TCP connection
	// .await because it’s asynchronous (Tokio runtime).
	// .with_context(...) adds extra error details if the connection fails (using anyhow or eyre error context).
    let tcp_stream = TcpStream::connect(addr)
        .await 
        .with_context(|| format!("Failed to connect TCP to {}", addr))?;

    // Validate domain for TLS
	// ServerName is required by Rustls to check the certificate’s Common Name (CN) 
	// or Subject Alternative Name (SAN) matches the domain.
    let server_name = ServerName::try_from(domain)
        .context("Invalid DNS name for TLS connection")?;

    // Perform TLS handshake
    let tls_stream = connector.connect(server_name, tcp_stream)
        .await
        .with_context(|| format!("TLS handshake failed with {}", domain))?;

    Ok(tls_stream)
}
