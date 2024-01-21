use crate::AppState;
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use base64::prelude::*;
use std::collections::HashMap;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}

type Cookies<'a> = HashMap<&'a str, Cookie<'a>>;
struct Cookie<'a> {
    name: &'a str,
    value: &'a str,
}

fn cookiejar_from_headers(headers: &HeaderMap) -> Cookies {
    match headers.get("cookie") {
        Some(values) => values
            .to_str()
            .unwrap()
            .split(',')
            .map(|c| {
                let s = c.trim().split_once('=').unwrap();
                let cookie = Cookie {
                    name: s.0,
                    value: s.1.split_once(';').unwrap_or((s.1, "")).0,
                };
                (cookie.name, cookie)
            })
            .collect::<Cookies>(),
        None => Cookies::new(),
    }
}

async fn decode(headers: HeaderMap) -> Response {
    let cookies = cookiejar_from_headers(&headers);
    let encoded_recipe = cookies.get("recipe").unwrap().value;
    let decoded_recipe =
        String::from_utf8(BASE64_STANDARD.decode(encoded_recipe).unwrap()).unwrap();
    decoded_recipe.into_response()
}

#[derive(serde::Deserialize)]
struct Input {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}
#[derive(serde::Serialize)]
struct Result {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

async fn bake(headers: HeaderMap) -> Response {
    let cookies = cookiejar_from_headers(&headers);
    let encoded_recipe_for_pantry = cookies.get("recipe").unwrap().value;
    let input_json =
        String::from_utf8(BASE64_STANDARD.decode(encoded_recipe_for_pantry).unwrap()).unwrap();
    let input = serde_json::from_str::<Input>(input_json.as_str()).unwrap();

    let mut max_cookies_per_ingredient: Vec<u64> = vec![];
    for (ingredient, amount) in &input.recipe {
        if amount == &0 {
            continue;
        }
        let pantry_amount = *input.pantry.get(ingredient).unwrap_or(&0);
        max_cookies_per_ingredient.push(if pantry_amount < *amount {
            0
        } else {
            pantry_amount / *amount
        });
    }

    let max_cookies = *max_cookies_per_ingredient.iter().min().unwrap_or(&0);

    let mut new_pantry = HashMap::new();
    for (ingredient, amount) in input.pantry {
        let recipe_amount = *input.recipe.get(&ingredient).unwrap_or(&0);
        new_pantry.insert(ingredient, amount - (recipe_amount * max_cookies));
    }

    let result = Result {
        cookies: max_cookies,
        pantry: new_pantry,
    };

    Json(result).into_response()
}
