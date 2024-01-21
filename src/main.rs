use axum::Router;

mod five;
mod four;
mod minus_one;
mod one;
mod six;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(minus_one::router())
        .nest("/1", one::router())
        .nest("/4", four::router())
        .nest("/5", five::router())
        .nest("/6", six::router());

    Ok(router.into())
}
