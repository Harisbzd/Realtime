mod analyze;
mod processor;
mod sensor;
mod transmitter;
mod types;

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::sensor::start_sensor_data_stream;
use crate::types::{SensorProcessor, Transmitter};

#[tokio::main]
async fn main() {
    let (sensor_tx, mut sensor_rx) = mpsc::channel(100);
    let (tx_tx, mut tx_rx) = mpsc::channel(100);
    let (log_tx, mut log_rx) = mpsc::channel::<String>(3000); 

    // Create and wrap log file
    let file = File::create("processing_times.csv").expect("Failed to create log file");
    let file = Arc::new(Mutex::new(file));

    // Spawn logger task
    let file_clone = Arc::clone(&file);
    let logger_handle = tokio::spawn(async move {
        {
            let mut f = file_clone.lock().unwrap();
            writeln!(
                f,
                "PacketID,SensorGenTime_ms,ProcessingTime_ms,TransmissionTime_ms"
            ).ok();
        }

        while let Some(entry) = log_rx.recv().await {
            let mut f = file_clone.lock().unwrap();
            writeln!(f, "{}", entry).ok();
        }

        println!("Logger task exiting.");
    });

    // Spawn sensor task
    tokio::spawn(start_sensor_data_stream(sensor_tx.clone()));
    drop(sensor_tx); // allow exit once stream ends

    // Spawn processor task
    let tx_tx_clone = tx_tx.clone();
    let log_tx_clone = log_tx.clone();
    let processor_handle = tokio::spawn(async move {
        let mut processor = SensorProcessor::new();

        while let Some((packet_id, packet, sensor_gen_time)) = sensor_rx.recv().await {
            let proc_start = Instant::now();
            let (filtered, anomalies) = processor.process(&packet);
            let proc_time = proc_start.elapsed().as_secs_f64() * 1000.0;

            // Send to transmitter
            tx_tx_clone
                .send((packet_id, filtered, anomalies, sensor_gen_time, proc_time))
                .await
                .ok();
        }

        drop(tx_tx_clone);
        drop(log_tx_clone); 
        println!("Processor task exiting.");
    });

    drop(tx_tx); 

    // Spawn transmitter task
    let log_tx_clone = log_tx.clone();
    let transmitter_handle = tokio::spawn(async move {
        let transmitter = Transmitter::new("amqp://127.0.0.1:5672/%2f").await;
        while let Some((packet_id, packet, _anomalies, sensor_gen_time, proc_time)) =
            tx_rx.recv().await
        {
            let tx_start = Instant::now();
            let payload = serde_json::to_vec(&packet).unwrap();
            transmitter.send_serialized(&payload).await;
            let tx_time = tx_start.elapsed().as_secs_f64() * 1000.0;

            // Send log to logger
            let log_line = format!(
                "{},{:.3},{:.3},{:.3}",
                packet_id, sensor_gen_time, proc_time, tx_time
            );
            log_tx_clone.send(log_line).await.ok();
        }

        println!("Transmitter task exiting.");
    });

    drop(log_tx); // let logger task exit when all logs are sent

    // Await all tasks
    let _ = processor_handle.await;
    let _ = transmitter_handle.await;
    let _ = logger_handle.await;

    // Run post-analysis
    if let Err(e) = analyze::analyze_log("processing_times.csv") {
        eprintln!("Analysis failed: {}", e);
    }

    println!("All tasks completed. Program exiting.");
}