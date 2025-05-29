
use crate::types::Transmitter;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};

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

        Self { channel }
    }

    pub async fn send_serialized(&self, payload: &[u8]) {
        let confirm = self.channel
            .basic_publish(
                "",
                "sensor_data",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default().with_delivery_mode(1), 
            )
            .await;
    
        if let Err(e) = confirm {
            eprintln!("Publish error: {:?}", e);
        }
    }
} 
