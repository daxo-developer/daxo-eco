//! Cryptographic utilities for signing and verifying sensor data.
//! Uses Ed25519 signatures and postcard serialization.
//! All operations are `no_std` compatible.

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use serde::Serialize;
use alloc::vec::Vec;
use crate::sensor_data::MeasurementPayload;

/// Key pair containing both secret and public keys.
pub struct KeyPair {
    pub public: VerifyingKey,
    pub secret: SigningKey,
}

impl KeyPair {
    /// Generate a new random keypair (only available when `std` feature is enabled)
    #[cfg(feature = "std")]
    pub fn generate() -> Self {
        use rand_core::OsRng;
        let secret = SigningKey::generate(&mut OsRng);
        let public = secret.verifying_key();
        KeyPair { public, secret }
    }

    /// Create a keypair from a 32-byte secret key (for bare-metal / deterministic use)
    pub fn from_secret_bytes(bytes: &[u8; 32]) -> Self {
        let secret = SigningKey::from_bytes(bytes);
        let public = secret.verifying_key();
        KeyPair { public, secret }
    }

    /// Return the public key as bytes.
    pub fn public_bytes(&self) -> Vec<u8> {
        self.public.to_bytes().to_vec()
    }
}

/// Sign binary data with the given keypair.
pub fn sign_data_binary(data: &[u8], keypair: &KeyPair) -> Vec<u8> {
    let signature = keypair.secret.sign(data);
    signature.to_bytes().to_vec()
}

/// Verify a complete MeasurementPayload (signature + public key).
pub fn verify_payload(payload: &MeasurementPayload) -> bool {
    if payload.signature.len() != 64 || payload.public_key.len() != 32 {
        return false;
    }
    let signature = match Signature::from_slice(&payload.signature) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let verifying_key = match VerifyingKey::try_from(&payload.public_key[..]) {
        Ok(k) => k,
        Err(_) => return false,
    };
    verifying_key.verify(&payload.data, &signature).is_ok()
}

/// Encode any serializable type into postcard bytes.
pub fn encode_sensor_data<T: Serialize>(data: &T) -> Vec<u8> {
    postcard::to_allocvec(data).expect("postcard serialization failed")
}
