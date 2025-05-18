mod processor;
mod sensor;
pub mod transmitter;
mod types;

use processor::SensorProcessor;
use sensor::start_sensor_data_stream;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use transmitter::Transmitter;
use tokio::sync::mpsc as tokio_mpsc;

#[tokio::main]
async fn main() {
    let (sensor_tx, mut sensor_rx) = tokio_mpsc::channel(100);
    let (tx_tx, mut tx_rx) = tokio_mpsc::channel(100);

    start_sensor_data_stream(sensor_tx);

    let mut processor = SensorProcessor::new();
    let transmitter = Transmitter::new("amqp://127.0.0.1:5672/%2f").await;

    // Spawn transmitter task
    tokio::spawn(async move {
        while let Some(packet) = tx_rx.recv().await {
            let tx_start = Instant::now();
            transmitter.send(&packet).await;
            let tx_time = tx_start.elapsed().as_secs_f64() * 1000.0;
            println!("Transmit time: {:.3} ms", tx_time);
        }
    });

    println!("🔄 Starting real-time processing...");

    let mut file = File::create("sensor_log.csv").expect("Failed to create log file");
    writeln!(
        file,
        "Packet,Force,Position,Temperature,Anomaly,ProcTime_ms"
    )
    .expect("Failed to write CSV header");

    let mut packet_count = 0;
    while let Some(packet) = sensor_rx.recv().await {
        packet_count += 1;

        let proc_start = Instant::now();
        let (filtered, anomalies) = processor.process(packet);
        let proc_time = proc_start.elapsed().as_secs_f64() * 1000.0;

        println!(
            "Packet {}:\n  Filtered force: {:.3}\n  position: {:.3}\n  temperature: {:.4}",
            packet_count, filtered.force, filtered.position, filtered.temperature
        );

        let anomaly_str = anomalies
            .iter()
            .map(|a| format!("{:?}", a.anomaly))
            .collect::<Vec<_>>()
            .join("; ");

        if tx_tx.send(filtered.clone()).await.is_err() {
            eprintln!("Failed to send packet to transmitter");
        }

        writeln!(
            file,
            "{},{:.3},{:.3},{:.4},\"{}\",{:.3}",
            packet_count,
            filtered.force,
            filtered.position,
            filtered.temperature,
            anomaly_str,
            proc_time
        )
        .expect("Failed to write to CSV file");
    }
}
