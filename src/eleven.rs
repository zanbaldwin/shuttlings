use crate::AppState;
use axum::{
    body::Bytes,
    extract::multipart::Multipart,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use image::{GenericImageView, Pixel};

const DECORATION_IMAGE: &[u8] = include_bytes!("../assets/11-decoration.png");

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/assets/decoration.png", get(image))
        .route("/red_pixels", post(red_pixels))
}

async fn image() -> Response {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("content-type", "image/png".parse().unwrap());
    (StatusCode::OK, headers, DECORATION_IMAGE).into_response()
}

async fn red_pixels(mut form: Multipart) -> Response {
    let image_data: Bytes;
    'field: {
        while let Some(field) = form.next_field().await.unwrap() {
            if field.name().unwrap() != "image" {
                continue;
            }
            image_data = field.bytes().await.unwrap();
            break 'field;
        }

        return (StatusCode::BAD_REQUEST, "No image field submitted.").into_response();
    }

    let image = image::load_from_memory(&image_data).unwrap();
    let num_magic_pixels = image
        .pixels()
        .filter(|(_x, _y, pixel)| {
            let rgb = pixel.to_rgb();
            let red = rgb.0[0] as u16;
            let green = rgb.0[1] as u16;
            let blue = rgb.0[2] as u16;
            red > green + blue
        })
        .count();

    num_magic_pixels.to_string().into_response()
}
