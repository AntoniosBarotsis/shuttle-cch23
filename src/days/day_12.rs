use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
  time::{Instant, SystemTime},
};

use anyhow::anyhow;
use axum::{
  extract::{Path, State},
  routing::{get, post},
  Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use serde_json::{json, Value};
use ulid::Ulid;
use uuid::Uuid;

use super::AppError;

pub fn get_routes() -> Router {
  let shared_state = Arc::new(RwLock::new(AppState {
    entries: HashMap::new(),
  }));

  Router::new()
    .route("/12/save/:id", post(task_1_put))
    .route("/12/load/:id", get(task_1_get))
    .route("/12/ulids", post(task_2))
    .route("/12/ulids/:weekday", post(task_3))
    .with_state(shared_state)
}

struct AppState {
  entries: HashMap<EntryId, Instant>,
}

type EntryId = String;

async fn task_1_put(
  State(state): State<Arc<RwLock<AppState>>>,
  Path(id): Path<String>,
) -> Result<(), AppError> {
  #[allow(clippy::unwrap_used)]
  let _ = state.write().unwrap().entries.insert(id, Instant::now());

  Ok(())
}

async fn task_1_get(
  State(state): State<Arc<RwLock<AppState>>>,
  Path(id): Path<String>,
) -> Result<String, AppError> {
  #[allow(clippy::unwrap_used)]
  let elapsed_seconds = state
    .read()
    .unwrap()
    .entries
    .get(&id)
    .ok_or_else(|| anyhow!("Entry \'{id}\' not found"))?
    .elapsed()
    .as_secs();

  Ok(elapsed_seconds.to_string())
}

fn parse_ulids(input: Vec<String>) -> impl DoubleEndedIterator<Item = Ulid> {
  input
    .into_iter()
    .map(|el| Ulid::from_string(&el))
    .filter_map(Result::ok)
}

async fn task_2(Json(payload): Json<Vec<String>>) -> Result<Json<Vec<String>>, AppError> {
  let uuids = parse_ulids(payload)
    .map(Uuid::from)
    .map(|el| el.to_string())
    .rev()
    .collect::<Vec<_>>();

  Ok(Json(uuids))
}

async fn task_3(
  Path(weekday): Path<String>,
  Json(payload): Json<Vec<String>>,
) -> Result<Json<Value>, AppError> {
  fn ulid_filter<F: FnMut(&&Ulid) -> bool>(ulids: &[Ulid], f: F) -> usize {
    ulids.iter().filter(f).count()
  }

  let weekday = weekday.parse::<u32>()?;
  let ulids = parse_ulids(payload).collect::<Vec<_>>();

  let christmas_eves = ulid_filter(&ulids, |el| {
    let date = DateTime::<Utc>::from(el.datetime());

    date.month() == 12 && date.day() == 24
  });
  let weekday = ulid_filter(&ulids, |el| {
    DateTime::<Utc>::from(el.datetime())
      .weekday()
      .num_days_from_monday()
      == weekday
  });
  let future = ulid_filter(&ulids, |el| {
    DateTime::<Utc>::from(el.datetime()) > DateTime::<Utc>::from(SystemTime::now())
  });
  let lsb = ulid_filter(&ulids, |el| el.0 & 1 == 1);

  let res = json!({
    "christmas eve": christmas_eves,
    "weekday": weekday,
    "in the future": future,
    "LSB is 1": lsb
  });

  Ok(Json(res))
}
