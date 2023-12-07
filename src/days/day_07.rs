#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use axum::{
  body::Body,
  error_handling::HandleError,
  http::{HeaderMap, Request, StatusCode},
  routing::get,
  Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};

pub fn get_routes() -> Router {
  let task_service = tower::service_fn(|req: Request<Body>| async move {
    let res = task_3(req.headers()).await?;
    Ok::<_, anyhow::Error>(res)
  });

  Router::new().route("/7/decode", get(task_1)).route_service(
    "/7/bake",
    HandleError::new(task_service, handle_anyhow_error),
  )
}

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("Something went wrong: {err}"),
  )
}

async fn task_1(headers: HeaderMap) -> String {
  let cookie = &headers
    .get("cookie")
    .expect("Cookie not found")
    .to_str()
    .unwrap()["result=".len()..];

  let decoded = general_purpose::STANDARD.decode(cookie).unwrap();

  String::from_utf8(decoded).unwrap()
}

#[derive(Debug, serde::Deserialize)]
struct CookieData {
  recipe: HashMap<String, u64>,
  pantry: HashMap<String, u64>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Ingredients {
  flour: u32,
  sugar: u32,
  butter: u32,
  #[serde(rename(deserialize = "baking powder"))]
  baking_powder: u32,
  #[serde(rename(deserialize = "chocolate chips"))]
  chocolate_chips: u32,
}

#[derive(Debug, serde::Serialize)]
struct CookieResponse {
  cookies: u32,
  pantry: Ingredients,
}

fn parse_cookie<T: for<'a> serde::Deserialize<'a>>(
  headers: &HeaderMap,
) -> Result<T, anyhow::Error> {
  let cookie = &headers.get("cookie").expect("Cookie not found").to_str()?["result=".len()..];
  let decoded = general_purpose::STANDARD.decode(cookie)?;
  let data: &str = std::str::from_utf8(&decoded)?;
  let parsed: T = serde_json::from_str(data)?;

  Ok(parsed)
}

async fn task_3(headers: &HeaderMap) -> Result<Json<Value>, anyhow::Error> {
  let parsed = parse_cookie::<CookieData>(headers)?;

  let recipe = parsed.recipe;
  let mut pantry = parsed.pantry;

  let cookies = recipe
    .iter()
    .fold(u64::MAX, |cookies, (ingredient, needed)| {
      let available = pantry.get(ingredient).unwrap_or(&0);
      cookies.min(available / needed)
    });

  for (key, pantry_value) in &mut pantry {
    *pantry_value -= cookies * recipe.get(key).unwrap_or(&0u64);
  }

  let res = json!({
    "cookies": cookies,
    "pantry": pantry
  });

  Ok(Json(res))
}
