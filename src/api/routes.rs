use std::sync::Arc;
use warp::Filter;

use crate::api::handlers::{ApiState, get_prices, get_rsi, get_health};

pub fn create_routes(state: Arc<ApiState>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let state_filter = warp::any().map(move || state.clone());

    let health = warp::path("health")
        .and(warp::get())
        .and_then(get_health);

    let prices = warp::path("prices")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_prices);

    let rsi = warp::path("rsi")
        .and(warp::get())
        .and(state_filter)
        .and_then(get_rsi);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    health
        .or(prices)
        .or(rsi)
        .with(cors)
}
