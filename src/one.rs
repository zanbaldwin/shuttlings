use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/*nums", get(xor_pow3))
}

async fn xor_pow3(Path(nums): Path<String>) -> Response {
    nums.split('/')
        .map(|num| num.parse::<i32>().unwrap())
        .reduce(|carry, item| carry ^ item)
        .unwrap()
        .pow(3)
        .to_string()
        .into_response()
}
