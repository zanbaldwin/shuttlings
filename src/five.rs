use crate::AppState;
use axum::{extract::Query, routing::post, Json, Router};
use serde_json::Value;
use std::{cmp::min, collections::HashMap};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(names))
}

async fn names(
    Query(params): Query<HashMap<String, usize>>,
    Json(names): Json<Vec<String>>,
) -> Json<Value> {
    let offset = *params.get("offset").unwrap_or(&0);

    let names = match params.get("limit") {
        Some(limit) => names[offset..min(names.len(), offset + limit)].to_vec(),
        None => names[offset..].to_vec(),
    };

    let names = if let Some(split) = params.get("split").copied() {
        serde_json::to_value(names.chunks(split).collect::<Vec<_>>())
    } else {
        serde_json::to_value(names)
    };

    Json(names.unwrap())
}
