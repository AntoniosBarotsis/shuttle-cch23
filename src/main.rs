#![allow(clippy::unused_async)]

pub mod days;

use axum::{http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
  "Hello, world!"
}

async fn internal_server_error() -> StatusCode {
  StatusCode::INTERNAL_SERVER_ERROR
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(internal_server_error))
    .merge(days::day_01::get_routes())
    .merge(days::day_04::get_routes())
    .merge(days::day_07::get_routes())
    .merge(days::day_06::get_routes());

  Ok(router.into())
}
