use axum::{routing::post, Json, Router};

pub fn router() -> Router {
    Router::new().route("/", post(count))
}

#[derive(serde::Serialize)]
struct Result {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    no_elf_shelf: usize,
}

async fn count(body: String) -> Json<Result> {
    let singular = body.matches("elf").count();
    // let elf_shelf = body.matches("elf on a shelf").count();
    let elf_shelf = char_windows(body.as_str(), "elf on a shelf".len())
        .filter(|slice| *slice == "elf on a shelf")
        .count();
    let no_elf_shelf = body.matches("shelf").count() - elf_shelf;

    Json(Result {
        elf: singular,
        shelf: elf_shelf,
        no_elf_shelf,
    })
}

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}
