//! SecureIoTOS Bootloader Unit Test Module
//!
//! License : Dual License
//!   - Apache 2.0 for open-source / personal use
//!   - Commercial license required for closed-source use
//!
//! Author  : Md Mahbubur Rahman
//! Project : https://m-a-h-b-u-b.github.io
//! GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS

#[cfg(test)]
mod tests {
    use p256::{
        ecdsa::{signature::Signer, signature::Verifier, SigningKey, VerifyingKey, Signature},
        pkcs8::DecodePrivateKey,
    };
    use sha2::{Sha256, Digest};

    /// Vendor public key (DER format, ASN.1 SubjectPublicKeyInfo)
    /// Extracted once using `openssl ec -in vendor.pem -pubout -outform DER > vendor_pub.der`
    const VENDOR_PUBLIC_KEY_DER: &[u8] = include_bytes!("vendor_pub.der");

    /// For testing only: vendor private key (PEM format)
    /// In real life, kept offline and NEVER in bootloader code
    const VENDOR_PRIVATE_KEY_PEM: &str = r#"
    -----BEGIN PRIVATE KEY-----
    MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgsGZWz9m7p6M5Zkqh
    yQkN9H7xSxPxxfdbdvC6T8cm7u+hRANCAASW/dgHZsxlrGrttCNwbm+H0ryfpZBa
    sDpzlbsjXBEe0J1gXm dCZ n/cswK4p8P5RaHEV+szmDzn+ONNDaC92q9P
    -----END PRIVATE KEY-----
    "#;

    /// Hash the firmware with SHA-256
    fn hash_firmware(firmware: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(firmware);
        hasher.finalize().to_vec()
    }

    /// Verify firmware using embedded vendor public key
    fn verify_signature(firmware: &[u8], signature: &Signature) -> bool {
        let verifying_key = VerifyingKey::from_public_key_der(VENDOR_PUBLIC_KEY_DER).unwrap();
        verifying_key.verify(&hash_firmware(firmware), signature).is_ok()
    }

    #[test]
    fn test_valid_signature() {
        let firmware = b"secureiot-firmware-v1.0";

        // Load vendor signing key
        let signing_key = SigningKey::from_pkcs8_pem(VENDOR_PRIVATE_KEY_PEM).unwrap();

        // Sign firmware
        let signature: Signature = signing_key.sign(&hash_firmware(firmware));

        // Verify with embedded DER public key
        assert!(
            verify_signature(firmware, &signature),
            "Valid firmware signed by vendor must verify"
        );
    }

    #[test]
    fn test_invalid_signature_wrong_key() {
        let firmware = b"secureiot-firmware-v1.0";

        // Attacker generates random key
        let wrong_key = SigningKey::random(rand::thread_rng());
        let signature: Signature = wrong_key.sign(&hash_firmware(firmware));

        // Bootloader rejects since vendor key doesn't match
        assert!(
            !verify_signature(firmware, &signature),
            "Firmware signed with non-vendor key must fail"
        );
    }

    #[test]
    fn test_tampered_firmware() {
        let original_firmware = b"secureiot-firmware-v1.0";
        let tampered_firmware = b"secureiot-firmware-v1.0-hacked";

        let signing_key = SigningKey::from_pkcs8_pem(VENDOR_PRIVATE_KEY_PEM).unwrap();
        let signature: Signature = signing_key.sign(&hash_firmware(original_firmware));

        assert!(
            !verify_signature(tampered_firmware, &signature),
            "Tampered firmware must not match vendor signature"
        );
    }
}
