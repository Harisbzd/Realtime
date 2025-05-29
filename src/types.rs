use lapin::Channel;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, time::SystemTime};

#[derive(Debug, Clone, Serialize)]
pub struct SensorPacket {
    pub timestamp: SystemTime,
    pub force: f32,
    pub position: f32,
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub enum AnomalyType {
    ForceSpike,
    TempSpike,
    PositionalError,
}

#[derive(Debug, Clone)]
pub struct AnomalyReport {
    pub anomaly: AnomalyType,
    pub packet: SensorPacket,
}

#[derive(Clone)]
pub struct Transmitter {
    pub(crate) channel: Channel,
}

#[derive(Clone)]
pub struct SensorProcessor {
    pub force_buffer: VecDeque<f32>,
    pub temp_buffer: VecDeque<f32>,
    pub pos_buffer: VecDeque<f32>,
}

#[derive(Debug, Deserialize)]
pub enum Feedback {
    UpdateForceThreshold(f32),
    UpdateTempThreshold(f32),
    UpdatePosThreshold(f32),
}
