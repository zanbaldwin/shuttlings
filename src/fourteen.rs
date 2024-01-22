use crate::AppState;
use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use htmlentity::entity::{CharacterSet, EncodeType, ICodedDataTrait};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/unsafe", post(unsafe_html))
        .route("/safe", post(safe_html))
}

#[derive(Deserialize)]
struct Input {
    content: String,
}

fn template(content: String) -> String {
    format!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        content
    )
}

async fn unsafe_html(Json(input): Json<Input>) -> Response {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    (StatusCode::OK, headers, template(input.content)).into_response()
}

async fn safe_html(Json(input): Json<Input>) -> Response {
    let content = htmlentity::entity::encode(
        input.content.as_bytes(),
        &EncodeType::NamedOrHex,
        &CharacterSet::SpecialCharsAndNonASCII,
    )
    .to_string()
    .unwrap();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    (StatusCode::OK, headers, template(content)).into_response()
}
