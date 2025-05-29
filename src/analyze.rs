use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use statrs::statistics::Statistics;

pub fn analyze_log(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut proc_times = Vec::new();
    let mut tx_times = Vec::new();
    let mut gen_times = Vec::new();

    for result in rdr.records().skip(1) {
        match result {
            Ok(record) => {
                let genration: f64 = record[1].parse().unwrap_or(0.0);
                let proc: f64 = record[2].parse().unwrap_or(0.0);
                let tx: f64 = record[3].parse().unwrap_or(0.0);
    
                gen_times.push(genration);
                proc_times.push(proc);
                tx_times.push(tx);
            }
            Err(e) => {
                eprintln!("⚠️ Failed to read a row: {}", e);
            }
        }
    }

    let packet_count = proc_times.len();
    let total_time_sec = (packet_count as f64 * 5.0) / 1000.0; // assuming 5ms interval

    let throughput = packet_count as f64 / total_time_sec;

    println!(" Throughput: {:.2} packets/sec", throughput);
    println!(" Jitter (Sensor Genration): {:.3} ms", gen_times.std_dev());
    println!(" Jitter (Processing): {:.3} ms", proc_times.std_dev());
    println!(" Jitter (Transmission): {:.3} ms", tx_times.std_dev());

    Ok(())
}