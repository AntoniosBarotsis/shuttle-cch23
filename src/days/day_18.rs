use std::collections::BTreeMap;

use super::day_13::{insert, MyState};
use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use sqlx::PgPool;

use super::AppError;

pub fn get_routes(pool: PgPool) -> Router {
  let state = MyState { pool };

  Router::new()
    .route("/18/reset", post(reset))
    .route("/18/orders", post(insert))
    .route("/18/regions", post(insert_region))
    .route("/18/regions/total", get(total))
    .route("/18/regions/top_list/:number", get(best))
    .with_state(state)
}

async fn reset(State(state): State<MyState>) -> Result<(), AppError> {
  let _ = sqlx::query!("DROP TABLE IF EXISTS regions")
    .execute(&state.pool)
    .await?;
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

  let _ = sqlx::query!(
    r"
    CREATE TABLE regions (
      id INT PRIMARY KEY,
      name VARCHAR(50)
    )"
  )
  .execute(&state.pool)
  .await?;

  Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct Region {
  id: i32,
  name: String,
}

#[derive(serde::Serialize, Debug)]
struct RegionTotal {
  region: String,
  total: i32,
}

#[derive(serde::Serialize, Debug)]
struct RegionBest {
  gift_name: Option<String>,
  region: String,
}

#[derive(serde::Serialize, Debug)]
struct RegionBestResp {
  region: String,
  top_gifts: Vec<String>,
}

async fn insert_region(
  State(state): State<MyState>,
  Json(payload): Json<Vec<Region>>,
) -> Result<impl IntoResponse, AppError> {
  // dbg!(&payload);
  for el in payload {
    let _ = sqlx::query!(
      "INSERT INTO regions (id, name) VALUES ($1, $2)",
      el.id,
      el.name,
    )
    .execute(&state.pool)
    .await?;
  }

  Ok(())
}

async fn total(State(state): State<MyState>) -> Result<Json<Vec<RegionTotal>>, AppError> {
  let res = sqlx::query_as!(
    RegionTotal,
    r#"SELECT 
        name as "region!", 
        SUM(quantity)::INT as "total!" 
      FROM 
        orders 
        JOIN regions ON orders.region_id = regions.id 
      GROUP BY 
        region_id, 
        name
      ORDER BY 
        name;
      "#
  )
  .fetch_all(&state.pool)
  .await?
  .into_iter()
  .collect::<Vec<_>>();

  Ok(Json(res))
}

async fn best(
  Path(number): Path<usize>,
  State(state): State<MyState>,
) -> Result<Json<Vec<RegionBestResp>>, AppError> {
  let res = sqlx::query_as!(
    RegionBest,
    r#"SELECT
      gift_name,
      name AS "region!"
    FROM orders
    RIGHT JOIN regions ON orders.region_id = regions.id
    GROUP BY region_id,
          gift_name,
          name
    ORDER BY SUM(quantity) DESC, gift_name;
      "#
  )
  .fetch_all(&state.pool)
  .await?
  .into_iter()
  .collect::<Vec<_>>();

  let mut tmp = BTreeMap::<String, Vec<String>>::new();

  for el in res {
    let list = tmp
      .entry(el.region)
      .and_modify(|e| {
        if e.len() < number {
          if let Some(gift_name) = el.gift_name.clone() {
            e.push(gift_name);
          }
        }
      })
      .or_default();

    if list.is_empty() && list.len() < number {
      if let Some(gift_name) = el.gift_name.clone() {
        list.push(gift_name);
      }
    }
  }

  let res = tmp
    .into_iter()
    .map(|el| RegionBestResp {
      region: el.0,
      top_gifts: el.1,
    })
    .collect::<Vec<_>>();

  Ok(Json(res))
}
