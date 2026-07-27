use reqwest::Client;
use daxo_eco_core::sensor_data::MeasurementPayload;

const SERVER_URL: &str = "http://localhost:8080/measurement";

pub async fn send_measurement_binary(
    data: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> Result<(), anyhow::Error> {
    let payload = MeasurementPayload {
        data: data.to_vec(),
        signature: signature.to_vec(),
        public_key: public_key.to_vec(),
    };
    let bytes = postcard::to_allocvec(&payload).expect("postcard serialization");
    let client = Client::new();
    let resp = client
        .post(SERVER_URL)
        .header("Content-Type", "application/octet-stream")
        .body(bytes)
        .send()
        .await?;
    if resp.status().is_success() {
        Ok(())
    } else {
        let text = resp.text().await.unwrap_or_default();
        Err(anyhow::anyhow!("Server error: {}", text))
    }
}
