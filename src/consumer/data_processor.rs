use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::models::{TradeData, RsiData};

#[derive(Debug, Clone)]
pub struct PriceHistory {
    pub symbol: String,
    pub prices: Vec<f64>,
    pub timestamps: Vec<DateTime<Utc>>,
}

impl PriceHistory {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            prices: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    pub fn add_price(&mut self, price: f64, timestamp: DateTime<Utc>) {
        self.prices.push(price);
        self.timestamps.push(timestamp);
        
        // Keep only last 100 prices for memory efficiency
        if self.prices.len() > 100 {
            self.prices.remove(0);
            self.timestamps.remove(0);
        }
    }

    pub fn calculate_rsi(&self, period: usize) -> Option<f64> {
        if self.prices.len() < period + 1 {
            return None;
        }

        let mut gains = Vec::new();
        let mut losses = Vec::new();

        for i in 1..self.prices.len() {
            let change = self.prices[i] - self.prices[i - 1];
            if change > 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }

        if gains.len() < period {
            return None;
        }

        // Calculate average gain and loss for the period
        let avg_gain = gains.iter().rev().take(period).sum::<f64>() / period as f64;
        let avg_loss = losses.iter().rev().take(period).sum::<f64>() / period as f64;

        if avg_loss == 0.0 {
            return Some(100.0);
        }

        let rs = avg_gain / avg_loss;
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        
        Some(rsi)
    }
}

#[derive(Clone)]
pub struct DataProcessor {
    price_histories: Arc<RwLock<HashMap<String, PriceHistory>>>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            price_histories: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn process_trade_data(&self, trade_data: TradeData) {
        let symbol = trade_data.symbol.clone();
        let price = trade_data.price;
        let timestamp = trade_data.timestamp;

        // Update price history
        {
            let mut histories = self.price_histories.write().await;
            let history = histories.entry(symbol.clone()).or_insert_with(|| {
                PriceHistory::new(symbol.clone())
            });
            history.add_price(price, timestamp);
        }

        // Calculate RSI if we have enough data
        let rsi = {
            let histories = self.price_histories.read().await;
            if let Some(history) = histories.get(&symbol) {
                history.calculate_rsi(14) // 14-period RSI
            } else {
                None
            }
        };

        if let Some(rsi_value) = rsi {
            let rsi_data = RsiData::new(symbol.clone(), rsi_value, 14);
            println!("📈 RSI calculated for {}: {:.2} ({:?})", 
                symbol, rsi_value, rsi_data.signal);
            
            // Here we would send RSI data to the RSI topic
            // For now, we'll just log it
        }
    }

    pub async fn get_latest_prices(&self) -> HashMap<String, f64> {
        let histories = self.price_histories.read().await;
        histories.iter()
            .filter_map(|(symbol, history)| {
                history.prices.last().map(|&price| (symbol.clone(), price))
            })
            .collect()
    }

    pub async fn get_latest_rsi(&self) -> HashMap<String, f64> {
        let histories = self.price_histories.read().await;
        histories.iter()
            .filter_map(|(symbol, history)| {
                history.calculate_rsi(14).map(|rsi| (symbol.clone(), rsi))
            })
            .collect()
    }
}
