#![allow(clippy::unused_async)]

pub mod days;

use axum::{http::StatusCode, routing::get, Router};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
  "Hello, world!"
}

async fn internal_server_error() -> StatusCode {
  StatusCode::INTERNAL_SERVER_ERROR
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres(
    local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
  )]
  pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  tracing_subscriber::fmt()
    .without_time()
    .with_line_number(true)
    .pretty()
    .init();

  sqlx::migrate!("./migrations/")
    .run(&pool)
    .await
    .expect("Error running DB migrations");

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(internal_server_error))
    .merge(days::day_01::get_routes())
    .merge(days::day_04::get_routes())
    .merge(days::day_05::get_routes())
    .merge(days::day_06::get_routes())
    .merge(days::day_07::get_routes())
    .merge(days::day_08::get_routes())
    .merge(days::day_11::get_routes())
    .merge(days::day_12::get_routes())
    .merge(days::day_13::get_routes(pool.clone()))
    .merge(days::day_14::get_routes())
    .merge(days::day_15::get_routes())
    .merge(days::day_18::get_routes(pool.clone()))
    .merge(days::day_19::get_routes())
    .merge(days::day_20::get_routes());

  Ok(router.into())
}
