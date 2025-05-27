use criterion::{criterion_group, criterion_main, Criterion};
use r1::types::SensorProcessor;
use r1::types::Transmitter;
use r1::sensor::generate_sensor_packet;
use r1::types::SensorPacket;
use serde_json;
use tokio::runtime::Runtime;
use std::time::SystemTime;

fn bench_processor(c: &mut Criterion) {
    let mut processor = SensorProcessor::new();
    let packet = SensorPacket {
        timestamp: SystemTime::now(),
        force: 16.5,
        position: 1.1,
        temperature: 32.0,
    };

    c.bench_function("processor_process", |b| {
        b.iter(|| {
            let _ = processor.process(&packet);
        });
    });
}

fn bench_send_raw(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let transmitter = rt.block_on(async {
        Transmitter::new("amqp://127.0.0.1:5672/%2f").await
    });

    let packet = generate_sensor_packet();
    let payload = serde_json::to_vec(&packet).unwrap();

    c.bench_function("transmitter_send_raw", |b| {
        b.iter(|| {
            rt.block_on(async {
                transmitter.send_raw(&payload).await;
            });
        });
    });
}

criterion_group!(benches, bench_processor, bench_send_raw);
criterion_main!(benches);