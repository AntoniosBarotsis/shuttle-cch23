use anyhow::anyhow;
use axum::{extract::Path, routing::get, Router};
use serde_json::Value;

use super::AppError;

pub fn get_routes() -> Router {
  Router::new()
    .route("/8/weight/:id", get(task_1))
    .route("/8/drop/:id", get(task_2))
}

async fn get_weight(id: u32) -> Result<f64, AppError> {
  let res = ureq::get(&format!("https://pokeapi.co/api/v2/pokemon/{id}"))
    .call()?
    .into_string()?;

  let root = serde_json::from_str::<Value>(&res)?;

  let weight = root
    .get("weight")
    .ok_or_else(|| anyhow!("Weight not found"))?
    .as_number()
    .and_then(serde_json::Number::as_f64)
    .ok_or_else(|| anyhow!("Weight not a number"))?;

  Ok(weight)
}

async fn task_1(Path(id): Path<u32>) -> Result<String, AppError> {
  let weight = get_weight(id).await?;

  let weight_kg = weight / 10.0;

  Ok(weight_kg.to_string())
}

async fn task_2(Path(id): Path<u32>) -> Result<String, AppError> {
  let weight = get_weight(id).await?;

  let weight_kg = weight / 10.0;

  #[allow(clippy::cast_precision_loss)]
  let res = (2.0f64 * 9.825 * 10.0).sqrt() * weight_kg;

  Ok(res.to_string())
}
