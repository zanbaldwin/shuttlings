use axum::Router;
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
    time::SystemTime,
};

mod eight;
mod eleven;
mod five;
mod four;
mod minus_one;
mod one;
mod seven;
mod six;
mod thirteen;
mod twelve;

type AppState = Arc<Mutex<State>>;
#[derive(Default)]
struct State {
    pub twelve_packages: HashMap<String, SystemTime>,
    pub thirteen_orders: BTreeMap<u32, thirteen::Order>,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(minus_one::router())
        .nest("/1", one::router())
        .nest("/4", four::router())
        .nest("/5", five::router())
        .nest("/6", six::router())
        .nest("/7", seven::router())
        .nest("/8", eight::router())
        .nest("/11", eleven::router())
        .nest("/12", twelve::router())
        .nest("/13", thirteen::router())
        .with_state(AppState::default());

    Ok(router.into())
}
