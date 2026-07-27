#![no_std]
#![no_main]

use daxo_eco_core::sensor_data::SensorData;
use daxo_eco_core::crypto::{KeyPair, sign_data_binary, encode_sensor_data};
use ed25519_dalek::SigningKey;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let secret = SigningKey::from_bytes(&[0u8; 32]);
    let public = secret.verifying_key();
    let keypair = KeyPair { public, secret };

    let data = SensorData {
        timestamp: "2026-07-26T15:00:00Z".into(),
        sequence_id: 1,
        pm25: 25.0,
        temperature: 22.0,
        humidity: 50.0,
        latitude: 50.45,
        longitude: 30.52,
    };
    let bytes = encode_sensor_data(&data);
    let _sig = sign_data_binary(&bytes, &keypair);
    loop {}
}
