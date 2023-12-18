use anyhow::anyhow;
use axum::{
  extract::State,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use super::AppError;

// use super::AppError;

pub fn get_routes(pool: PgPool) -> Router {
  let state = MyState { pool };

  Router::new()
    .route("/13/sql", get(task_1))
    .route("/13/reset", post(reset))
    .route("/13/orders", post(insert))
    .route("/13/orders/total", get(total))
    .route("/13/orders/popular", get(popular))
    .with_state(state)
}

#[derive(Clone)]
pub(crate) struct MyState {
  pub(crate) pool: PgPool,
}

async fn task_1(State(state): State<MyState>) -> Result<String, AppError> {
  let query = sqlx::query!("SELECT 20231213 number")
    .fetch_one(&state.pool)
    .await?
    .number
    .ok_or_else(|| anyhow!("Database exploded"))?;

  Ok(query.to_string())
}

async fn reset(State(state): State<MyState>) -> Result<(), AppError> {
  let _ = sqlx::query!("DROP TABLE IF EXISTS orders")
    .execute(&state.pool)
    .await?;

  let _ = sqlx::query!(
    r"
  CREATE TABLE orders (
    id INT PRIMARY KEY,
    region_id INT,
    gift_name VARCHAR(50),
    quantity INT
  )"
  )
  .execute(&state.pool)
  .await?;

  Ok(())
}

#[derive(Deserialize, Debug)]
pub(crate) struct Order {
  id: i32,
  region_id: i32,
  gift_name: String,
  quantity: i32,
}

pub(crate) async fn insert(
  State(state): State<MyState>,
  Json(payload): Json<Vec<Order>>,
) -> Result<impl IntoResponse, AppError> {
  // dbg!(&payload);
  for el in payload {
    let _ = sqlx::query!(
      "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
      el.id,
      el.region_id,
      el.gift_name,
      el.quantity
    )
    .execute(&state.pool)
    .await?;
  }

  Ok(())
}

async fn total(State(state): State<MyState>) -> Result<Json<Value>, AppError> {
  let total = sqlx::query!("SELECT SUM(quantity) total from orders")
    .fetch_one(&state.pool)
    .await?
    .total
    .ok_or_else(|| anyhow!("Database exploded"))?;

  let res = json!({
    "total": total,
  });

  Ok(Json(res))
}

async fn popular(State(state): State<MyState>) -> Result<String, AppError> {
  let popular = sqlx::query!("SELECT gift_name, SUM(quantity) total from orders GROUP BY gift_name ORDER BY SUM(quantity) DESC")
    .fetch_optional(&state.pool)
    .await?.map(|el| el.gift_name);

  Ok(
    json!({
      "popular": popular,
    })
    .to_string(),
  )
}
