use rand::Rng;

pub struct Reading {
    pub pm25: f32,
    pub temperature: f32,
    pub humidity: f32,
}

pub async fn read_sensors() -> Reading {
    let mut rng = rand::thread_rng();
    Reading {
        pm25: rng.gen_range(15.0..55.0),
        temperature: rng.gen_range(18.0..28.0),
        humidity: rng.gen_range(30.0..70.0),
    }
}