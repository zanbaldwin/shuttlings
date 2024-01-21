use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use std::time::SystemTime;
use ulid::{DecodeError, Ulid};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/save/*str_id", post(save_string))
        .route("/load/*str_id", get(load_string))
        .route("/ulids", post(ulids))
        .route("/ulids/:weekday", post(weekday))
}

async fn save_string(Path(str_id): Path<String>, State(state): State<AppState>) -> Response {
    let packages = &mut state.lock().unwrap().twelve_packages;
    packages.insert(str_id, SystemTime::now());
    StatusCode::OK.into_response()
}

async fn load_string(Path(str_id): Path<String>, State(state): State<AppState>) -> Response {
    let packages = &mut state.lock().unwrap().twelve_packages;
    let Some(time) = packages.get(&str_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let elapsed = time.elapsed().unwrap().as_secs();
    elapsed.to_string().into_response()
}

async fn ulids(Json(ulids): Json<Vec<String>>) -> Response {
    let Ok(ulids) = ulids
        .into_iter()
        .map(|ulid_string| ulid::Ulid::from_string(&ulid_string))
        .rev()
        .collect::<Result<Vec<Ulid>, DecodeError>>()
    else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    Json(
        ulids
            .into_iter()
            .map(|ulid| Uuid::from(ulid).to_string())
            .collect::<Vec<String>>(),
    )
    .into_response()
}

#[derive(serde::Serialize, Debug)]
struct WeekdayResult {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    future: usize,
    #[serde(rename = "LSB is 1")]
    lsb: usize,
}

async fn weekday(Path(weekday): Path<u32>, Json(ulids): Json<Vec<String>>) -> Response {
    let Ok(ulids) = ulids
        .into_iter()
        .map(|ulid_string| ulid::Ulid::from_string(&ulid_string))
        .rev()
        .collect::<Result<Vec<Ulid>, DecodeError>>()
    else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let christmas_eve = ulids
        .iter()
        .filter(|ulid| {
            let time: DateTime<Utc> = ulid.datetime().into();
            time.month() == 12 && time.day() == 24
        })
        .count();
    let weekday = ulids
        .iter()
        .filter(|ulid| {
            let time: DateTime<Utc> = ulid.datetime().into();
            // .weekday() is 1-indexed, but the weekday path param is 0-indexed
            (time.weekday().number_from_monday() - 1) == weekday
        })
        .count();
    let future = ulids
        .iter()
        .filter(|ulid| ulid.timestamp_ms() as u128 > now)
        .count();
    let lsb = ulids
        .iter()
        .filter(|ulid| u128::from(**ulid) & 1 == 1)
        .count();

    Json(WeekdayResult {
        christmas_eve,
        weekday,
        future,
        lsb,
    })
    .into_response()
}
