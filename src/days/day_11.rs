use axum::{
  extract::Multipart,
  http::header,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use std::{fs, io::Cursor};

use super::AppError;

pub fn get_routes() -> Router {
  Router::new()
    .route("/11/assets/decoration.png", get(task_1))
    .route("/11/red_pixels", post(task_2))
}

async fn task_1() -> Result<impl IntoResponse, AppError> {
  let buffer = fs::read("assets/decoration.png")?;
  let length = buffer.len();

  let headers = [
    (header::CONTENT_TYPE, String::from("image/png")),
    (header::CONTENT_LENGTH, length.to_string()),
  ];

  Ok((headers, buffer))
}

async fn task_2(mut multipart: Multipart) -> Result<String, AppError> {
  let mut reds = 0u64;

  // No need for a while since there's only 1 image/field
  if let Some(field) = multipart.next_field().await? {
    let data = field.bytes().await?;

    let img = ImageReader::new(Cursor::new(&data))
      .with_guessed_format()?
      .decode()?;

    let count = img
      .pixels()
      .map(|(_i, _j, Rgba([r, g, b, _]))| u64::from(r > g.saturating_add(b)))
      .sum::<u64>();

    reds += count;
  }

  Ok(reds.to_string())
}
