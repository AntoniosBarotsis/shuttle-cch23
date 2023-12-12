use axum::{http::StatusCode, response::IntoResponse};

pub mod day_01;
pub mod day_04;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_11;
pub mod day_12;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Something went wrong: {}", self.0),
    )
      .into_response()
  }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
  fn from(err: E) -> Self {
    Self(err.into())
  }
}
