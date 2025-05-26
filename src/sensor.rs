use crate::types::SensorPacket;
use rand::{rng, Rng};
use std::time::{Duration, Instant, SystemTime};



fn generate_sensor_value(base: f32, variance: f32) -> f32 {
    rng().random_range(base - variance..=base + variance)
}


pub fn generate_sensor_packet() -> SensorPacket {
    let mut force = generate_sensor_value(10.0, 2.0);
    let mut position = generate_sensor_value(5.0, 1.0);
    let mut temperature = generate_sensor_value(25.0, 0.5);

    // Inject anomalies randomly (5%)
    let mut rng = rng();
    let spike_chance: f32 = rng.random();
    if spike_chance < 0.05 {
        match rng.random_range(0..3) {
            0 => force = 16.0 + rng.random_range(0.0..5.0),
            1 => temperature = 31.0 + rng.random_range(0.0..5.0),
            2 => position = rng.random_range(0.0..1.5),
            _ => {}
        }
    }

    SensorPacket {
        timestamp: SystemTime::now(),
        force,
        position,
        temperature,
    }
}


pub fn start_sensor_data_stream(tx: tokio::sync::mpsc::Sender<SensorPacket>) {
    tokio::spawn(async move {
        let interval = Duration::from_millis(5);
        let mut next_tick = Instant::now();

        for _ in 0..3000 {
            next_tick += interval;
            let packet = super::sensor::generate_sensor_packet();
            if tx.send(packet).await.is_err() {
                eprintln!("Receiver dropped. Stopping sensor stream.");
                break;
            }

            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });
}