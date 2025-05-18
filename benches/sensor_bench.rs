use criterion::{criterion_group, criterion_main, Criterion};
use r1::sensor::generate_sensor_packet;

fn bench_generate_sensor(c: &mut Criterion) {
    c.bench_function("generate_sensor_packet", |b| {
        b.iter(|| {
            let _ = std::hint::black_box(generate_sensor_packet());
        });
    });
}

criterion_group!(benches, bench_generate_sensor);
criterion_main!(benches);