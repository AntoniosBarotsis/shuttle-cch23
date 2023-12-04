use axum::{routing::post, Json, Router};

use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router {
  Router::new()
    .route("/4/strength", post(task_1))
    .route("/4/contest", post(task_2))
}

#[derive(Deserialize, Debug)]
struct Deer1 {
  strength: i32,
}

#[derive(Deserialize, Debug)]
struct Deer2 {
  name: String,
  strength: i32,
  speed: f32,
  height: i32,
  antler_width: i32,
  snow_magic_power: i32,
  favorite_food: String,
  #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
  candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
struct DeersResponse {
  fastest: String,
  tallest: String,
  magician: String,
  consumer: String,
}

async fn task_1(Json(payload): Json<Vec<Deer1>>) -> String {
  payload
    .iter()
    .map(|el| el.strength)
    .sum::<i32>()
    .to_string()
}

#[allow(clippy::unwrap_used)]
async fn task_2(Json(payload): Json<Vec<Deer2>>) -> Json<DeersResponse> {
  let fastest = payload
    .iter()
    .max_by(|a, b| a.speed.total_cmp(&b.speed))
    .unwrap();

  let tallest = payload
    .iter()
    .max_by(|a, b| a.height.cmp(&b.height))
    .unwrap();

  let magician = payload
    .iter()
    .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
    .unwrap();

  let consumer = payload
    .iter()
    .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
    .unwrap();

  let res = DeersResponse {
    fastest: format!(
      "Speeding past the finish line with a strength of {} is {}",
      fastest.strength, fastest.name
    ),
    tallest: format!(
      "{} is standing tall with his {} cm wide antlers",
      tallest.name, tallest.antler_width
    ),
    magician: format!(
      "{} could blast you away with a snow magic power of {}",
      magician.name, magician.snow_magic_power
    ),
    consumer: format!(
      "{} ate lots of candies, but also some {}",
      consumer.name, consumer.favorite_food
    ),
  };

  Json(res)
}
