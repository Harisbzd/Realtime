use crate::types::{AnomalyReport, AnomalyType, SensorPacket};
use std::collections::VecDeque;
use std::time::Instant;

const WINDOW: usize = 1;

pub struct SensorProcessor {
    force_buffer: VecDeque<f32>,
    temp_buffer: VecDeque<f32>,
    pos_buffer: VecDeque<f32>,
}

impl SensorProcessor {
    pub fn new() -> Self {
        Self {
            force_buffer: VecDeque::with_capacity(WINDOW),
            temp_buffer: VecDeque::with_capacity(WINDOW),
            pos_buffer: VecDeque::with_capacity(WINDOW),
        }
    }

    pub fn process(&mut self, packet: SensorPacket) -> (SensorPacket, Vec<AnomalyReport>) {
        let start = Instant::now();

        let mut anomalies = Vec::new();

        if packet.force > 15.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::ForceSpike,
                packet: packet.clone(),
            });
        }

        if packet.temperature > 30.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::TempSpike,
                packet: packet.clone(),
            });
        }

        if packet.position < 2.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::PositionalError,
                packet: packet.clone(),
            });
        }

        let f = Self::avg(&mut self.force_buffer, packet.force);
        let t = Self::avg(&mut self.temp_buffer, packet.temperature);
        let p = Self::avg(&mut self.pos_buffer, packet.position);

        let smoothed = SensorPacket {
            timestamp: packet.timestamp,
            force: f,
            temperature: t,
            position: p,
        };

        let elapsed = start.elapsed();
        if cfg!(debug_assertions) {
            println!("Processing time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
        }

        (smoothed, anomalies)
    }

    fn avg(buffer: &mut VecDeque<f32>, val: f32) -> f32 {
        if buffer.len() == WINDOW {
            buffer.pop_front();
        }
        buffer.push_back(val);
        buffer.iter().copied().sum::<f32>() / buffer.len() as f32
    }
}
