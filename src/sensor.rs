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

    let mut rng = rng();
    let spike_chance: f32 = rng.random();
    if spike_chance < 0.20 {
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

pub async fn start_sensor_data_stream(tx: tokio::sync::mpsc::Sender<(usize, SensorPacket, f64)>) {
    let interval = Duration::from_millis(5);
    let mut next_tick = Instant::now();

    for packet_id in 1..=3000 {
        next_tick += interval;

        let gen_start = Instant::now();
        let packet = generate_sensor_packet();
        let gen_time = gen_start.elapsed().as_secs_f64() * 1000.0; // ms

        if tx.send((packet_id, packet, gen_time)).await.is_err() {
            eprintln!("Receiver dropped. Stopping sensor stream.");
            break;
        }

        if let Some(remaining) = next_tick.checked_duration_since(Instant::now()) {
            tokio::time::sleep(remaining).await;
        }
    }
}