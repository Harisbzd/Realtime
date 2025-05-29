use criterion::{criterion_group, criterion_main, Criterion};
use r1::types::SensorProcessor;
use r1::sensor::generate_sensor_packet;
use r1::types::Transmitter;
use tokio::runtime::Runtime;
use serde_json;

pub fn bench_full_pipeline(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let transmitter = rt.block_on(async {
        Transmitter::new("amqp://127.0.0.1:5672/%2f").await
    });

    let mut processor = SensorProcessor::new();

    c.bench_function("full_sensor_process_transmit_pipeline", |b| {
        b.iter(|| {
            rt.block_on(async {
                let packet = generate_sensor_packet();                    // Simulate sensor
                let (processed, _) = processor.process(&packet);          // Apply filter & detect anomalies
                let payload = serde_json::to_vec(&processed).unwrap();         // Serialize
                transmitter.send_serialized(&payload).await;                            // Transmit to RabbitMQ
            });
        });
    });
}

criterion_group!(benches, bench_full_pipeline);
criterion_main!(benches);