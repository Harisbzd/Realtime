use lapin::Channel;
use serde::Serialize;
use std::{collections::VecDeque, time::SystemTime};

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
    #[allow(dead_code)]
    pub packet: &'a SensorPacket,
}
pub struct Transmitter {
    pub(crate) channel: Channel, 
}

pub struct SensorProcessor {
    pub force_buffer: VecDeque<f32>,
    pub temp_buffer: VecDeque<f32>,
    pub pos_buffer: VecDeque<f32>,
}
