use criterion::{criterion_group, criterion_main, Criterion};
use r1::transmitter::Transmitter;
use r1::sensor::generate_sensor_packet; 
use tokio::runtime::Runtime;

pub fn bench_transmitter_send_raw(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let transmitter = rt.block_on(async {
        Transmitter::new("amqp://127.0.0.1:5672/%2f").await
    });

    let packet = generate_sensor_packet();
    let payload = serde_json::to_vec(&packet).unwrap();

    c.bench_function("transmitter_send_raw_realistic_payload", |b| {
        b.iter(|| {
            rt.block_on(async {
                transmitter.send_raw(&payload).await;
            });
        });
    });
}

criterion_group!(benches, bench_transmitter_send_raw);
criterion_main!(benches);