use std::sync::Arc;
use tokio::sync::RwLock;
use warp::reply::json;
use warp::Reply;

use crate::consumer::DataProcessor;

pub struct ApiState {
    pub data_processor: Arc<RwLock<DataProcessor>>,
}

impl ApiState {
    pub fn new(data_processor: DataProcessor) -> Self {
        Self {
            data_processor: Arc::new(RwLock::new(data_processor)),
        }
    }
}

pub async fn get_prices(state: Arc<ApiState>) -> Result<impl Reply, warp::Rejection> {
    let processor = state.data_processor.read().await;
    let prices = processor.get_latest_prices().await;
    Ok(json(&prices))
}

pub async fn get_rsi(state: Arc<ApiState>) -> Result<impl Reply, warp::Rejection> {
    let processor = state.data_processor.read().await;
    let rsi_values = processor.get_latest_rsi().await;
    Ok(json(&rsi_values))
}

pub async fn get_health() -> Result<impl Reply, warp::Rejection> {
    Ok(json(&serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}
