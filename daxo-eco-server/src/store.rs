use std::collections::HashMap;
use daxo_eco_core::sensor_data::MeasurementPayload;
use daxo_eco_core::sensor_data::SensorData;

#[derive(serde::Serialize)]
pub struct StoredMeasurement {
    pub data: SensorData,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl From<MeasurementPayload> for StoredMeasurement {
    fn from(p: MeasurementPayload) -> Self {
        let data = postcard::from_bytes(&p.data).expect("valid sensor data");
        StoredMeasurement {
            data,
            signature: p.signature,
            public_key: p.public_key,
        }
    }
}

pub struct Store {
    measurements: Vec<StoredMeasurement>,
    last_seq: HashMap<Vec<u8>, u64>, // public_key -> last sequence
}

impl Store {
    pub fn new() -> Self {
        Store {
            measurements: Vec::new(),
            last_seq: HashMap::new(),
        }
    }

    pub fn insert(&mut self, payload: MeasurementPayload) -> Result<(), String> {
        let data: SensorData = match postcard::from_bytes(&payload.data) {
            Ok(d) => d,
            Err(_) => return Err("invalid sensor data".to_string()),
        };
        let pk = payload.public_key.clone();
        let seq = data.sequence_id;

        if let Some(&last) = self.last_seq.get(&pk) {
            if seq <= last {
                return Err("replay attack".to_string());
            }
        }
        self.last_seq.insert(pk, seq);
        self.measurements.push(StoredMeasurement {
            data,
            signature: payload.signature,
            public_key: payload.public_key,
        });
        Ok(())
    }

    pub fn latest(&self, n: usize) -> Vec<&StoredMeasurement> {
        let start = if self.measurements.len() > n {
            self.measurements.len() - n
        } else {
            0
        };
        self.measurements[start..].iter().collect()
    }
}