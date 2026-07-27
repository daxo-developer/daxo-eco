# Daxo Eco - Trusted Air Quality Monitoring

Bare‑metal capable core with Ed25519 signing and replay protection.  
Uses **postcard** for deterministic binary serialization.

## Run

1. Start server:
   `cd daxo-eco-server && cargo run`

2. Open http://localhost:8080

3. Run node:
   `cd daxo-eco-node && cargo run`

## Architecture

- `daxo-eco-core`: no_std, calibration, crypto, postcard serialization.
- Node: reads sensors, calibrates, signs with Ed25519, stores sequence in file (simulated NVS).
- Server: verifies signature, checks sequence_id, stores, exports to OpenAQ.

## Security

- Ed25519 signature over postcard-encoded SensorData prevents tampering.
- Sequence ID (persisted to disk) prevents replay attacks across reboots.
- Public key identifies device; server maintains last seen sequence.

## API

- POST `/measurement` - binary postcard-encoded MeasurementPayload.
- GET `/api/latest` - JSON list of latest measurements.
- GET `/api/export/openaq` - exports latest 100 measurements in OpenAQ format.

## Next steps

- Replace simulated sensors with real I2C/UART drivers.
- Port to ESP32‑C3 / STM32 using embassy‑rs.
- Replace file‑based seq persistence with flash storage.