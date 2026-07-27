//! Benchmark comparing JSON and Postcard serialization performance.
//! This binary measures both payload size and serialization speed.
//! It is meant to be run on the host (std environment) to obtain
//! quantitative metrics for the research paper.

use daxo_eco_core::sensor_data::SensorData;
use daxo_eco_core::crypto::encode_sensor_data;
use std::time::Instant;

fn main() {
    // Create a typical measurement sample (same as real sensor data)
    let data = SensorData {
        timestamp: "2026-07-26T15:00:00Z".to_string(),
        sequence_id: 1,
        pm25: 25.0,
        temperature: 22.0,
        humidity: 50.0,
        latitude: 50.45,
        longitude: 30.52,
    };

    // ---------- Payload size comparison ----------
    let json_bytes = serde_json::to_vec(&data).unwrap();
    println!("JSON size: {} bytes", json_bytes.len());

    let postcard_bytes = encode_sensor_data(&data);
    println!("Postcard size: {} bytes", postcard_bytes.len());

    // ---------- Serialization speed (Postcard) ----------
    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = encode_sensor_data(&data);
    }
    let dur = start.elapsed();
    println!("Postcard 10k iterations: {:?}", dur);
    println!(
        "Average: {:.2} µs",
        dur.as_micros() as f64 / 10_000.0
    );

    // ---------- Serialization speed (JSON) ----------
    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = serde_json::to_vec(&data).unwrap();
    }
    let dur = start.elapsed();
    println!("JSON 10k iterations: {:?}", dur);
    println!(
        "Average: {:.2} µs",
        dur.as_micros() as f64 / 10_000.0
    );
}
