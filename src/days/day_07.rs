#![allow(clippy::unwrap_used)]

use std::{string::FromUtf8Error, collections::HashMap};

use anyhow::anyhow;
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
  recipe: Ingredients,
  pantry: Ingredients,
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

async fn task_2(headers: &HeaderMap) -> Result<Json<CookieResponse>, anyhow::Error> {
  let cookie = &headers.get("cookie").expect("Cookie not found").to_str()?["result=".len()..];

  let decoded = general_purpose::STANDARD.decode(cookie)?;

  let data = String::from_utf8(decoded)?;

  let mut parsed: CookieData = serde_json::from_str(&data)?;

  let mut cookies = 0;
  loop {
    let flour = parsed.recipe.flour < parsed.pantry.flour;
    let sugar = parsed.recipe.sugar < parsed.pantry.sugar;
    let butter = parsed.recipe.butter < parsed.pantry.butter;
    let baking_powder = parsed.recipe.baking_powder < parsed.pantry.baking_powder;
    let chocolate_chips = parsed.recipe.chocolate_chips < parsed.pantry.chocolate_chips;

    if flour && sugar && butter && baking_powder && chocolate_chips {
      cookies += 1;

      parsed.pantry.flour -= parsed.recipe.flour;
      parsed.pantry.sugar -= parsed.recipe.sugar;
      parsed.pantry.butter -= parsed.recipe.butter;
      parsed.pantry.baking_powder -= parsed.recipe.baking_powder;
      parsed.pantry.chocolate_chips -= parsed.recipe.chocolate_chips;
    } else {
      break;
    }
  }

  Ok(Json(CookieResponse {
    cookies,
    pantry: parsed.pantry,
  }))
}

async fn task_3(headers: &HeaderMap) -> Result<Json<Value>, anyhow::Error> {
  let cookie = &headers.get("cookie").expect("Cookie not found").to_str()?["result=".len()..];
  let decoded = general_purpose::STANDARD.decode(cookie)?;
  let data = String::from_utf8(decoded)?;
  let parsed: Value = serde_json::from_str(&data)?;

  if let (Some(Value::Object(recipe)), Some(Value::Object(pantry))) =
    (parsed.get("recipe"), parsed.get("pantry"))
  {
    let mut cookies = 0;

    let mut pantry = pantry.clone();

    loop {
      let all_valid = recipe.iter().all(|(recipe_key, recipe_value)| {
        pantry
          .get(recipe_key)
          .map_or(false, |pantry_value| match (recipe_value, pantry_value) {
            (Value::Number(recipe_amt), Value::Number(pantry_amt)) => {
              pantry_amt.as_f64() > recipe_amt.as_f64()
            }
            _ => false,
          })
      });

      if all_valid {
        cookies += 1;

        recipe.iter().for_each(|(recipe_key, recipe_value)| {
          let pantry_value = pantry.get_mut(recipe_key).unwrap();

          if let (Value::Number(recipe_amt), Value::Number(pantry_amt)) =
            (recipe_value, &pantry_value)
          {
            *pantry_value =
              Value::Number((pantry_amt.as_i64().unwrap() - recipe_amt.as_i64().unwrap()).into());
          }
        });
      } else {
        break;
      }
    }

    let res = json!({
      "cookies": cookies,
      "pantry": pantry
    });

    return Ok(Json(res));
  }

  Err(anyhow!("Both `recipe` and `pantry` need to be present."))
}
