use axum::{routing::post, Json, Router};
use serde::Serialize;

pub fn get_routes() -> Router {
  Router::new().route("/6", post(task))
  // .route("/4/contest", post(task_2))
}

// #[derive(Serialize, Debug)]
// struct ElfCount {
//   #[serde(rename(serialize = "elf"))]
//   count: usize,
// }

// async fn task(Path(payload): Path<String>) -> Json<ElfCount> {
//   let count = payload.matches("elf").count();

//   Json(ElfCount { count })
// }

#[derive(Serialize, Debug)]
struct ElfCount {
  #[serde(rename(serialize = "elf"))]
  count: usize,
  #[serde(rename(serialize = "elf on a shelf"))]
  on_shelf: usize,
  #[serde(rename(serialize = "shelf with no elf on it"))]
  no_shelf: usize,
}

async fn task(payload: String) -> Json<ElfCount> {
  let count = payload.matches("elf").count();
  let on_shelf = payload.matches("elf on a shelf").count();
  let no_shelf = payload.matches("shelf").count() - on_shelf;

  Json(ElfCount {
    count,
    on_shelf,
    no_shelf,
  })
}
