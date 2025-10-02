use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsiData {
    pub id: String,
    pub symbol: String,
    pub rsi_value: f64,
    pub timestamp: DateTime<Utc>,
    pub period: u32,
    pub signal: RsiSignal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RsiSignal {
    Overbought,
    Oversold,
    Neutral,
}

impl RsiData {
    pub fn new(symbol: String, rsi_value: f64, period: u32) -> Self {
        let signal = match rsi_value {
            rsi if rsi >= 70.0 => RsiSignal::Overbought,
            rsi if rsi <= 30.0 => RsiSignal::Oversold,
            _ => RsiSignal::Neutral,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            symbol,
            rsi_value,
            timestamp: Utc::now(),
            period,
            signal,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
