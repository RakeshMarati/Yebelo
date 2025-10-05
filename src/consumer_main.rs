mod models;
mod consumer;
mod api;

use std::sync::Arc;

use consumer::{TradingConsumer, DataProcessor};
use api::{ApiState, create_routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Trading Data Consumer...");
    
    // Configuration - Read from environment variables
    let brokers = std::env::var("KAFKA_BROKERS")
        .unwrap_or_else(|_| "localhost:19092".to_string());
    let group_id = "trading-consumer-group";
    let api_port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);
    
    // Initialize data processor
    let data_processor = DataProcessor::new();
    let api_state = Arc::new(ApiState::new(data_processor.clone()));
    
    // Initialize consumer
    let consumer = TradingConsumer::new(brokers, group_id)?;
    consumer.subscribe_to_trade_data().await?;
    
    println!("📡 Connected to Redpanda at {}", brokers);
    println!("🌐 Starting API server on port {}", api_port);
    
    // Start API server
    let api_routes = create_routes(api_state.clone());
    let api_server = warp::serve(api_routes)
        .run(([0, 0, 0, 0], api_port));
    
    // Start consumer in background
    let consumer_task = tokio::spawn(async move {
        if let Err(e) = consumer.consume_messages().await {
            eprintln!("❌ Consumer error: {}", e);
        }
    });
    
    // Start API server in background
    let api_task = tokio::spawn(api_server);
    
    println!("✅ Consumer and API server started successfully!");
    println!("📊 API endpoints:");
    println!("   - Health: http://localhost:{}/health", api_port);
    println!("   - Prices: http://localhost:{}/prices", api_port);
    println!("   - RSI: http://localhost:{}/rsi", api_port);
    println!("⏰ Processing messages... (press Ctrl+C to stop)\n");
    
    // Wait for either task to complete
    tokio::select! {
        _ = consumer_task => {
            println!("Consumer task completed");
        }
        _ = api_task => {
            println!("API server task completed");
        }
    }
    
    Ok(())
}
