use crate::types::SensorPacket;
use lapin::{options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Transmitter {
    channel: Arc<Mutex<Channel>>,
}

impl Transmitter {
    pub async fn new(uri: &str) -> Self {
        let conn = Connection::connect(uri, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        let channel = conn.create_channel().await.expect("Create channel failed");

        channel
            .queue_declare(
                "sensor_data",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Queue declare failed");

        Self {
            channel: Arc::new(Mutex::new(channel)),
        }
    }

  
    pub async fn send(&self, packet: &SensorPacket) {
        let payload = serde_json::to_vec(packet).expect("Serialization failed");
        self.send_raw(&payload).await;
    }

    pub async fn send_raw(&self, payload: &[u8]) {
        let channel = self.channel.lock().await;

        channel
            .basic_publish(
                "",
                "sensor_data",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
            .expect("Publish failed");
            
    }
}