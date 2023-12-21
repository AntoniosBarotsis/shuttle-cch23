use axum::{extract::Query, routing::post, Json, Router};
use std::collections::HashMap;

use super::AppError;

pub fn get_routes() -> Router {
  Router::new().route("/5", post(task))
}

async fn task(
  Query(params): Query<HashMap<String, usize>>,
  Json(payload): Json<Vec<String>>,
) -> Result<String, AppError> {
  let offset = *params.get("offset").unwrap_or(&0);
  let limit = *params.get("limit").unwrap_or(&payload.len());
  let split = params.get("split");

  let payload = payload
    .into_iter()
    .skip(offset)
    .take(limit)
    .collect::<Vec<_>>();

  split.map_or_else(
    || Ok(serde_json::to_string(&payload).expect("Serialize")),
    |split| {
      let payload = payload.chunks(*split).collect::<Vec<_>>();

      Ok(serde_json::to_string(&payload).expect("Serialize"))
    },
  )
}
