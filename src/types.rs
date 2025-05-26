use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize)]
pub struct SensorPacket {
    pub timestamp: SystemTime,
    pub force: f32,
    pub position: f32,
    pub temperature: f32,
}

#[derive(Debug)]
pub enum AnomalyType {
    ForceSpike,
    TempSpike,
    PositionalError,
}

#[derive(Debug)]
pub struct AnomalyReport<'a> {
    pub anomaly: AnomalyType,
    pub packet: &'a SensorPacket,
}