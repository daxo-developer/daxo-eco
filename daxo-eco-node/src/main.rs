mod network;
mod sensors;
mod persist;

use tokio::time::{interval, Duration};
use chrono::Utc;
use daxo_eco_core::{
    sensor_data::SensorData,
    calibration::calibrate_pm,
    crypto::{KeyPair, sign_data_binary, encode_sensor_data},
    sequence::next_sequence,
};
use network::send_measurement_binary;
use persist::{load_seq, save_seq};

const SEQ_FILE: &str = "seq_state.bin";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = KeyPair::generate();
    let pub_bytes = keypair.public_bytes();
    eprintln!("Public key: {}", hex::encode(&pub_bytes));

    let mut seq_id = load_seq(SEQ_FILE).unwrap_or(0);
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        seq_id = next_sequence(seq_id);
        if let Err(e) = save_seq(SEQ_FILE, seq_id) {
            eprintln!("Failed to persist seq: {}", e);
        }

        let raw = sensors::read_sensors().await;
        let pm_corr = calibrate_pm(raw.pm25, raw.humidity);

        let measurement = SensorData {
            timestamp: Utc::now().to_rfc3339(),
            sequence_id: seq_id,
            pm25: pm_corr,
            temperature: raw.temperature,
            humidity: raw.humidity,
            latitude: 50.4501,
            longitude: 30.5234,
        };

        let data_bytes = encode_sensor_data(&measurement);
        let signature = sign_data_binary(&data_bytes, &keypair);

        if let Err(e) = send_measurement_binary(&data_bytes, &signature, &pub_bytes).await {
            eprintln!("Send error: {}", e);
        } else {
            eprintln!("Sent seq={} PM={:.1}", seq_id, measurement.pm25);
        }
    }
}
