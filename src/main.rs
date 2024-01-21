use axum::Router;

mod minus_one;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().merge(minus_one::router());

    Ok(router.into())
}
