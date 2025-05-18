use lapin::{
    options::*, types::FieldTable, Connection, ConnectionProperties, Channel,
};
use futures_util::stream::StreamExt;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct SensorPacket {
    timestamp: std::time::SystemTime,
    force: f32,
    position: f32,
    temperature: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to RabbitMQ
    let conn = Connection::connect("amqp://127.0.0.1:5672/%2f", ConnectionProperties::default()).await?;
    let channel: Channel = conn.create_channel().await?;

    // Declare the queue
    channel.queue_declare(
        "sensor_data",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    ).await?;

    println!("📡 Waiting for sensor data...");

    // Start consuming from the queue
    let mut consumer = channel
        .basic_consume(
            "sensor_data",
            "student_b_test",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut counter = 0; // Initialize counter

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let packet: SensorPacket = serde_json::from_slice(&delivery.data)?;
            counter += 1;

            println!("📥 Packet {} received: {:?}", counter, packet);

            // Acknowledge the message
            channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await?;
            
        }
    }

    println!("✅ Stream ended. Total packets received: {}", counter);

    Ok(())
}