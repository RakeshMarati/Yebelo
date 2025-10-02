use rand::Rng;
use std::collections::HashMap;

use crate::models::{TradeData, RsiData, TradeSide};

pub struct DataGenerator {
    symbols: Vec<String>,
    base_prices: HashMap<String, f64>,
    rsi_values: HashMap<String, f64>,
}

impl DataGenerator {
    pub fn new() -> Self {
        let symbols = vec![
            "AAPL".to_string(),
            "GOOGL".to_string(),
            "MSFT".to_string(),
            "TSLA".to_string(),
            "AMZN".to_string(),
            "NVDA".to_string(),
            "META".to_string(),
            "NFLX".to_string(),
        ];

        let mut base_prices = HashMap::new();
        let mut rsi_values = HashMap::new();

        for symbol in &symbols {
            base_prices.insert(symbol.clone(), rand::thread_rng().gen_range(50.0..500.0));
            rsi_values.insert(symbol.clone(), rand::thread_rng().gen_range(20.0..80.0));
        }

        Self {
            symbols,
            base_prices,
            rsi_values,
        }
    }

    pub fn generate_trade_data(&mut self) -> TradeData {
        let mut rng = rand::thread_rng();
        let symbol = self.symbols[rng.gen_range(0..self.symbols.len())].clone();
        
        // Get current base price and add some volatility
        let base_price = *self.base_prices.get(&symbol).unwrap();
        let volatility = rng.gen_range(-0.05..0.05); // ±5% volatility
        let price = base_price * (1.0 + volatility);
        
        // Update base price for next trade
        self.base_prices.insert(symbol.clone(), price);
        
        let volume = rng.gen_range(100..10000);
        let side = if rng.gen_bool(0.5) { TradeSide::Buy } else { TradeSide::Sell };
        let exchange = match rng.gen_range(0..3) {
            0 => "NYSE".to_string(),
            1 => "NASDAQ".to_string(),
            _ => "BATS".to_string(),
        };

        TradeData::new(symbol, price, volume, side, exchange)
    }

    pub fn generate_rsi_data(&mut self) -> RsiData {
        let mut rng = rand::thread_rng();
        let symbol = self.symbols[rng.gen_range(0..self.symbols.len())].clone();
        
        // Get current RSI and add some movement
        let current_rsi = *self.rsi_values.get(&symbol).unwrap();
        let rsi_change = rng.gen_range(-2.0..2.0);
        let rsi_value = (current_rsi + rsi_change).clamp(0.0, 100.0);
        
        // Update RSI for next calculation
        self.rsi_values.insert(symbol.clone(), rsi_value);
        
        let period = 14; // Standard RSI period

        RsiData::new(symbol, rsi_value, period)
    }
}
