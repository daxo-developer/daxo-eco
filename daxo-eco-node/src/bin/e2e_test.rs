//! End-to-end security test for Daxo Eco Server.
//! Tests:
//! 1. Legit request → 200 OK
//! 2. Replay attack (same sequence_id) → 409 Conflict
//! 3. Tampered data (change pm25 without re-signing) → 401 Unauthorized

use daxo_eco_core::{
    sensor_data::SensorData,
    crypto::{KeyPair, sign_data_binary, encode_sensor_data},
};
use reqwest;
use postcard;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_url = "http://127.0.0.1:8080/measurement";

    // 1. Generate keypair
    let keypair = KeyPair::generate();
    let pub_bytes = keypair.public_bytes();

    // 2. Build a legitimate measurement
    let data = SensorData {
        timestamp: Utc::now().to_rfc3339(),
        sequence_id: 100,
        pm25: 25.0,
        temperature: 22.0,
        humidity: 50.0,
        latitude: 50.4501,
        longitude: 30.5234,
    };
    let data_bytes = encode_sensor_data(&data);
    let signature = sign_data_binary(&data_bytes, &keypair);

    // Build payload
    let payload = daxo_eco_core::sensor_data::MeasurementPayload {
        data: data_bytes.clone(),
        signature: signature.clone(),
        public_key: pub_bytes.clone(),
    };
    let payload_bytes = postcard::to_allocvec(&payload).expect("serialization failed");

    let client = reqwest::Client::new();

    // ----- Test 1: Legit request -----
    println!("[1] Sending legit request...");
    let resp = client
        .post(server_url)
        .header("Content-Type", "application/octet-stream")
        .body(payload_bytes.clone())
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    println!("    Status: {} – {}", status, text);
    assert_eq!(status, 200, "Legit request failed");

    // ----- Test 2: Replay attack (same payload) -----
    println!("[2] Sending replay (same payload)...");
    let resp = client
        .post(server_url)
        .header("Content-Type", "application/octet-stream")
        .body(payload_bytes.clone())
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    println!("    Status: {} – {}", status, text);
    assert_eq!(status, 409, "Replay should be rejected with 409");

    // ----- Test 3: Tampered data (change pm25 to 999.9, keep signature) -----
    println!("[3] Sending tampered data (pm25=999.9, same signature)...");
    let mut tampered_data = data.clone();
    tampered_data.pm25 = 999.9;
    let tampered_bytes = encode_sensor_data(&tampered_data);
    // Note: signature is still for the original data!
    let tampered_payload = daxo_eco_core::sensor_data::MeasurementPayload {
        data: tampered_bytes,
        signature: signature.clone(), // invalid for tampered data
        public_key: pub_bytes.clone(),
    };
    let tampered_payload_bytes = postcard::to_allocvec(&tampered_payload).expect("serialization failed");
    let resp = client
        .post(server_url)
        .header("Content-Type", "application/octet-stream")
        .body(tampered_payload_bytes)
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    println!("    Status: {} – {}", status, text);
    assert_eq!(status, 401, "Tampered data should be rejected with 401");

    println!("\n✅ All security tests passed!");
    Ok(())
}
