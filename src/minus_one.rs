use axum::{http::StatusCode, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(server_error))
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn server_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}
