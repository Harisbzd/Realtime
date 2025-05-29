use std::collections::VecDeque;
use crate::types::{AnomalyReport, AnomalyType, SensorPacket, SensorProcessor};

const WINDOW: usize = 5;



impl SensorProcessor {
    pub fn new() -> Self {
        Self {
            force_buffer: VecDeque::with_capacity(WINDOW),
            temp_buffer: VecDeque::with_capacity(WINDOW),
            pos_buffer: VecDeque::with_capacity(WINDOW),
        }
    }

    pub fn process(&mut self, packet: &SensorPacket) -> (SensorPacket, Vec<AnomalyReport>) {
        let mut anomalies = Vec::new();

        if packet.force > 15.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::ForceSpike,
                packet: packet.clone(), // cloned here
            });
        }

        if packet.temperature > 30.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::TempSpike,
                packet: packet.clone(), // cloned here
            });
        }

        if packet.position > 2.0 {
            anomalies.push(AnomalyReport {
                anomaly: AnomalyType::PositionalError,
                packet: packet.clone(), // cloned here
            });
        }

        let f = Self::avg(&mut self.force_buffer, packet.force);
        let t = Self::avg(&mut self.temp_buffer, packet.temperature);
        let p = Self::avg(&mut self.pos_buffer, packet.position);

        let has_anomaly = !anomalies.is_empty();
        let smoothed = if has_anomaly {
            packet.clone()
        } else {
            SensorPacket {
                timestamp: packet.timestamp,
                force: f,
                temperature: t,
                position: p,
            }
        };

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