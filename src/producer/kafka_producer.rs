use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use rdkafka::util::Timeout;
use std::time::Duration;

use crate::models::{TradeData, RsiData};

pub struct TradingProducer {
    producer: FutureProducer,
    trade_topic: String,
    rsi_topic: String,
}

impl TradingProducer {
    pub fn new(brokers: &str, trade_topic: &str, rsi_topic: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("acks", "all")
            .set("retries", "3")
            .set("retry.backoff.ms", "100");

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

        let producer: FutureProducer = config.create()?;

        Ok(Self {
            producer,
            trade_topic: trade_topic.to_string(),
            rsi_topic: rsi_topic.to_string(),
        })
    }

    pub async fn send_trade_data(&self, trade_data: &TradeData) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = trade_data.to_json()?;
        let key = &trade_data.symbol;
        
        let record = FutureRecord::to(&self.trade_topic)
            .key(key)
            .payload(&json_data);

        match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok(_) => {
                println!("✅ Sent trade data: {} - {} @ ${:.2}", 
                    trade_data.symbol, 
                    format!("{:?}", trade_data.side), 
                    trade_data.price
                );
                Ok(())
            }
            Err((e, _)) => {
                eprintln!("❌ Failed to send trade data: {}", e);
                Err(e.into())
            }
        }
    }

    pub async fn send_rsi_data(&self, rsi_data: &RsiData) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = rsi_data.to_json()?;
        let key = &rsi_data.symbol;
        
        let record = FutureRecord::to(&self.rsi_topic)
            .key(key)
            .payload(&json_data);

        match self.producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok(_) => {
                println!("📊 Sent RSI data: {} - RSI: {:.2} ({:?})", 
                    rsi_data.symbol, 
                    rsi_data.rsi_value,
                    rsi_data.signal
                );
                Ok(())
            }
            Err((e, _)) => {
                eprintln!("❌ Failed to send RSI data: {}", e);
                Err(e.into())
            }
        }
    }

    pub async fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.producer.flush(Duration::from_secs(10))?;
        Ok(())
    }
}
