use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    pub id: String,
    pub symbol: String,
    pub price: f64,
    pub volume: u64,
    pub timestamp: DateTime<Utc>,
    pub side: TradeSide,
    pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

impl TradeData {
    pub fn new(symbol: String, price: f64, volume: u64, side: TradeSide, exchange: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            price,
            volume,
            timestamp: Utc::now(),
            side,
            exchange,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
