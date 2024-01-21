use axum::extract::Json;
use axum::response::{IntoResponse, Response};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SleighReindeer {
    #[serde(rename = "name")]
    _name: String,
    strength: i32,
}

#[derive(Deserialize)]
struct ContestReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten: i32,
}
#[derive(Serialize)]
struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/strength", post(reindeer_strength))
        .route("/contest", post(reindeer_candy_contest))
}

async fn reindeer_strength(Json(reindeer): Json<Vec<SleighReindeer>>) -> Response {
    reindeer
        .iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i32>()
        .to_string()
        .into_response()
}

async fn reindeer_candy_contest(Json(reindeer): Json<Vec<ContestReindeer>>) -> Response {
    let fastest = get_reindeer_with_max_attr(&reindeer, |r| r.speed);
    let tallest = get_reindeer_with_max_attr(&reindeer, |r| r.height as f32);
    let magician = get_reindeer_with_max_attr(&reindeer, |r| r.snow_magic_power as f32);
    let consumer = get_reindeer_with_max_attr(&reindeer, |r| r.candies_eaten as f32);

    serde_json::to_string(&ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
    .unwrap()
    .into_response()
}

fn get_reindeer_with_max_attr(
    reindeer: &[ContestReindeer],
    get: impl Fn(&ContestReindeer) -> f32,
) -> &ContestReindeer {
    reindeer
        .iter()
        .reduce(|a, b| if get(a) > get(b) { a } else { b })
        .unwrap()
}
