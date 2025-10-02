mod models;
mod producer;

use producer::{TradingProducer, DataGenerator};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Trading Data Producer...");
    
    // Configuration
    let brokers = "localhost:19092";
    let trade_topic = "trade-data";
    let rsi_topic = "rsi-data";
    
    // Initialize producer
    let producer = TradingProducer::new(brokers, trade_topic, rsi_topic)?;
    let mut data_generator = DataGenerator::new();
    
    println!("📡 Connected to Redpanda at {}", brokers);
    println!("📊 Producing data to topics: {} and {}", trade_topic, rsi_topic);
    println!("⏰ Starting data generation (press Ctrl+C to stop)...\n");
    
    // Main data generation loop
    let mut trade_counter = 0;
    let mut rsi_counter = 0;
    
    loop {
        // Generate and send trade data
        let trade_data = data_generator.generate_trade_data();
        if let Err(e) = producer.send_trade_data(&trade_data).await {
            eprintln!("❌ Trade data error: {}", e);
        } else {
            trade_counter += 1;
        }
        
        // Generate and send RSI data (every 5th iteration)
        if trade_counter % 5 == 0 {
            let rsi_data = data_generator.generate_rsi_data();
            if let Err(e) = producer.send_rsi_data(&rsi_data).await {
                eprintln!("❌ RSI data error: {}", e);
            } else {
                rsi_counter += 1;
            }
        }
        
        // Print stats every 10 trades
        if trade_counter % 10 == 0 {
            println!("📈 Stats - Trades: {}, RSI: {}", trade_counter, rsi_counter);
        }
        
        // Wait before next iteration
        sleep(Duration::from_millis(500)).await;
    }
}
