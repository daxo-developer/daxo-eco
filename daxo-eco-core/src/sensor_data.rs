use serde::{Deserialize, Serialize};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub timestamp: String,
    pub sequence_id: u64,
    pub pm25: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementPayload {
    pub data: Vec<u8>,           // postcard-encoded SensorData
    pub signature: Vec<u8>,      // Ed25519 signature (64 bytes)
    pub public_key: Vec<u8>,     // Ed25519 public key (32 bytes)
}