use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use std::time::Duration;
use tokio::time::timeout;

use crate::models::TradeData;
use crate::consumer::DataProcessor;

pub struct TradingConsumer {
    consumer: StreamConsumer,
    data_processor: DataProcessor,
}

impl TradingConsumer {
    pub fn new(brokers: &str, group_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest");

        // Add SASL authentication if environment variables are set
        // Only use SASL_SSL if all required variables are present
        if let (Ok(security_protocol), Ok(sasl_mechanism), Ok(sasl_username), Ok(sasl_password)) = (
            std::env::var("KAFKA_SECURITY_PROTOCOL"),
            std::env::var("KAFKA_SASL_MECHANISM"),
            std::env::var("KAFKA_SASL_USERNAME"),
            std::env::var("KAFKA_SASL_PASSWORD")
        ) {
            println!("🔐 Using SASL authentication");
            config.set("security.protocol", security_protocol);
            config.set("sasl.mechanism", sasl_mechanism);
            config.set("sasl.username", sasl_username);
            config.set("sasl.password", sasl_password);
        } else {
            println!("🔓 Using PLAINTEXT connection (no SASL)");
            config.set("security.protocol", "PLAINTEXT");
        }

        let consumer: StreamConsumer = config.create()?;

        let data_processor = DataProcessor::new();

        Ok(Self {
            consumer,
            data_processor,
        })
    }

    pub async fn subscribe_to_trade_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.consumer.subscribe(&["trade-data"])?;
        println!("📡 Subscribed to trade-data topic");
        Ok(())
    }

    pub async fn consume_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Starting message consumption...");
        
        loop {
            match timeout(Duration::from_secs(1), self.consumer.recv()).await {
                Ok(Ok(message)) => {
                    if let Some(payload) = message.payload() {
                        match serde_json::from_slice::<TradeData>(payload) {
                            Ok(trade_data) => {
                                println!("📊 Processing trade: {} - {} @ ${:.2}", 
                                    trade_data.symbol, 
                                    format!("{:?}", trade_data.side), 
                                    trade_data.price
                                );
                                
                                // Process the trade data
                                self.data_processor.process_trade_data(trade_data).await;
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to parse trade data: {}", e);
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("❌ Consumer error: {}", e);
                }
                Err(_) => {
                    // Timeout - continue loop
                }
            }
        }
    }
}
