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
    start_sensor_data_stream(sensor_tx);

    let mut processor = SensorProcessor::new();
    let transmitter = Transmitter::new("amqp://127.0.0.1:5672/%2f").await;

    let mut file = File::create("sensor_log.csv").expect("Failed to create log file");
    writeln!(
        file,
        "Packet,Force,Position,Temperature,Anomaly,SensorGen_ms,ProcTime_ms,TxTime_ms"
    )
    .expect("Failed to write CSV header");

    let mut packet_count = 0;
    while let Some((packet, sensor_time)) = sensor_rx.recv().await {
        packet_count += 1;

        // ⏱️ Processing
        let proc_start = Instant::now();
        let (filtered, anomalies) = processor.process(&packet);
        let proc_time = proc_start.elapsed().as_secs_f64() * 1000.0;

        // ⏱️ Transmission (inline)
        let tx_start = Instant::now();
        transmitter.send(&filtered).await;
        let tx_time = tx_start.elapsed().as_secs_f64() * 1000.0;

        // 📝 Anomaly description
        let anomaly_str = anomalies
            .iter()
            .map(|a| format!("{:?}", a.anomaly))
            .collect::<Vec<_>>()
            .join("; ");

        // 📝 Log to CSV
        writeln!(
            file,
            "{},{:.3},{:.3},{:.4},\"{}\",{:.3},{:.3},{:.3}",
            packet_count,
            filtered.force,
            filtered.position,
            filtered.temperature,
            anomaly_str,
            sensor_time,
            proc_time,
            tx_time
        )
        .expect("Failed to write to CSV file");
    }
}