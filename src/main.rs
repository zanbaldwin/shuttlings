use axum::Router;

mod eight;
mod five;
mod four;
mod minus_one;
mod one;
mod seven;
mod six;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(minus_one::router())
        .nest("/1", one::router())
        .nest("/4", four::router())
        .nest("/5", five::router())
        .nest("/6", six::router())
        .nest("/7", seven::router())
        .nest("/8", eight::router());

    Ok(router.into())
}
