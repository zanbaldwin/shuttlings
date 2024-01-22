use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sql", get(sql))
        .route("/reset", post(reset))
        .route("/orders", post(orders))
        .route("/orders/total", get(total))
        .route("/orders/popular", get(popular))
}

async fn sql() -> &'static str {
    "20231213"
}

#[derive(Deserialize)]
pub struct Order {
    id: u32,
    gift_name: String,
    quantity: u32,
}

async fn reset(State(state): State<AppState>) -> Response {
    let mut state = state.lock().unwrap();
    state.thirteen_orders.clear();
    StatusCode::OK.into_response()
}

async fn orders(State(state): State<AppState>, Json(orders): Json<Vec<Order>>) -> Response {
    let mut state = state.lock().unwrap();
    orders.into_iter().for_each(|order| {
        state.thirteen_orders.insert(order.id, order);
    });
    StatusCode::OK.into_response()
}

async fn total(State(state): State<AppState>) -> Response {
    let state = state.lock().unwrap();
    let total: u32 = state
        .thirteen_orders
        .iter()
        .map(|(_id, order)| order.quantity)
        .sum();
    Json(json!({ "total": total})).into_response()
}

async fn popular(State(state): State<AppState>) -> Response {
    let mut popular: HashMap<String, u32> = HashMap::new();
    let state = state.lock().unwrap();

    state.thirteen_orders.iter().for_each(|(_id, order)| {
        *popular.entry(order.gift_name.clone()).or_insert(0) += order.quantity
    });
    let mut toy = popular.iter().reduce(|a, b| if a.1 > b.1 { a } else { b });

    // Double check that it's the most popular, and not the most equally popular.
    if toy.is_some()
        && popular
            .iter()
            .filter(|(_name, count)| *count == toy.unwrap().1)
            .count()
            > 1
    {
        toy = None;
    }

    let toy = toy.map(|(name, _count)| name);
    Json(json!({"popular": toy})).into_response()
}
