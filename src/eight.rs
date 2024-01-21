use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

const GRAVATATIONAL_CONSTANT_EARTH: f64 = 9.825;
const DROP_HEIGHT: f64 = 10.0;

pub fn router() -> Router {
    Router::new()
        .route("/weight/:pokemon_id", get(pokemon_weight))
        .route("/drop/:pokemon_id", get(drop_pokemon))
}

#[derive(serde::Deserialize)]
struct Pokemon {
    #[serde(rename = "name")]
    _name: String,
    weight: u32,
}

async fn get_pokemon(pokemon_id: u32) -> Result<Pokemon, ()> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{pokemon_id}");
    let body = match reqwest::get(&url).await {
        Ok(response) => response.text().await.unwrap(),
        Err(_) => return Err(()),
    };
    Ok(serde_json::from_str::<Pokemon>(body.as_str()).unwrap())
}

async fn pokemon_weight(Path(pokemon_id): Path<u32>) -> Response {
    let Ok(pokemon) = get_pokemon(pokemon_id).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error with Pokemon API").into_response();
    };
    let pokemon_weight_in_kg = (pokemon.weight as f64) / 10.0;
    format!("{:.1}", pokemon_weight_in_kg).into_response()
}

async fn drop_pokemon(Path(pokemon_id): Path<u32>) -> Response {
    let Ok(pokemon) = get_pokemon(pokemon_id).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error with Pokemon API").into_response();
    };
    let pokemon_weight_in_kg = (pokemon.weight as f64) / 10.0;
    let momentum = momentum_of_mass_falling_from_height(pokemon_weight_in_kg, DROP_HEIGHT);
    format!("{:.4}", momentum).into_response()
}

fn momentum_of_mass_falling_from_height(mass: f64, height: f64) -> f64 {
    let velocity = (2.0 * GRAVATATIONAL_CONSTANT_EARTH * height).sqrt();
    let momentum = mass * velocity;
    momentum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Took me longer than I'd like to admit that I needed to use f64 instead of
    // f32 due to a lack of precision. I should have written a unit test from
    // the start, but nooooo I had to go down a rabbit hole of maths equations
    // when I'd already gotten them right already.
    fn test_momentum() {
        // Should equal 13_316.953 ish.
        let momentum = momentum_of_mass_falling_from_height(950.0, 10.0);
        eprintln!("{momentum:.4}");
        assert!(momentum > 13316.953480432378 - 0.001);
        assert!(momentum < 13316.953480432378 + 0.001);
    }
}
