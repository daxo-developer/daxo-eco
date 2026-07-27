mod store;

use actix_web::{web, App, HttpServer, HttpResponse, Error};
use std::sync::Mutex;
use daxo_eco_core::sensor_data::MeasurementPayload;
use daxo_eco_core::crypto::verify_payload;
use store::Store;
use serde::Serialize;

#[derive(Serialize)]
struct OpenAqLocation {
    pub location: String,
    pub parameter: String,
    pub value: f32,
    pub unit: String,
    pub coordinates: Coordinates,
}

#[derive(Serialize)]
struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

async fn submit_measurement(
    body: web::Bytes,
    store: web::Data<Mutex<Store>>,
) -> Result<HttpResponse, Error> {
    let payload: MeasurementPayload = match postcard::from_bytes(&body) {
        Ok(p) => p,
        Err(_) => return Ok(HttpResponse::BadRequest().body("invalid postcard")),
    };

    if !verify_payload(&payload) {
        return Ok(HttpResponse::Unauthorized().body("invalid signature"));
    }

    let mut store = store.lock().unwrap();
    if let Err(replay) = store.insert(payload) {
        return Ok(HttpResponse::Conflict().body(replay));
    }

    Ok(HttpResponse::Ok().body("accepted"))
}

async fn get_latest(store: web::Data<Mutex<Store>>) -> Result<HttpResponse, Error> {
    let store = store.lock().unwrap();
    let latest = store.latest(100);
    Ok(HttpResponse::Ok().json(latest))
}

async fn export_openaq(store: web::Data<Mutex<Store>>) -> Result<HttpResponse, Error> {
    let store = store.lock().unwrap();
    let latest = store.latest(100);
    let openaq_data: Vec<OpenAqLocation> = latest
        .iter()
        .map(|p| {
            let pub_key_short = hex::encode(&p.public_key[..8]);
            OpenAqLocation {
                location: format!("Kyiv-Daxo-{}", pub_key_short),
                parameter: "pm25".to_string(),
                value: p.data.pm25,
                unit: "µg/m³".to_string(),
                coordinates: Coordinates {
                    latitude: p.data.latitude,
                    longitude: p.data.longitude,
                },
            }
        })
        .collect();
    Ok(HttpResponse::Ok().json(openaq_data))
}

// --- Главная страница (HTML встроен) ---
async fn index() -> HttpResponse {
    let html = include_str!("../../daxo-eco-web/index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let store = web::Data::new(Mutex::new(Store::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .route("/", web::get().to(index))
            .route("/measurement", web::post().to(submit_measurement))
            .route("/api/latest", web::get().to(get_latest))
            .route("/api/export/openaq", web::get().to(export_openaq))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
