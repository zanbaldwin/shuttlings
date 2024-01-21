use axum::Router;

mod four;
mod minus_one;
mod one;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(minus_one::router())
        .nest("/1", one::router())
        .nest("/4", four::router());

    Ok(router.into())
}
