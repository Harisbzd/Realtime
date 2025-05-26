use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use r1::processor::SensorProcessor;
use r1::sensor::generate_sensor_packet; 

fn bench_processor_with_realistic_data(c: &mut Criterion) {
    let mut processor = SensorProcessor::new();

    c.bench_function("process_realistic_sensor_packet", |b| {
        b.iter(|| {
            let packet = generate_sensor_packet(); 
            let _ = processor.process(black_box(&packet));
        });
    });
}

criterion_group!(benches, bench_processor_with_realistic_data);
criterion_main!(benches);