use anyhow::anyhow;
use axum::{extract::Path, routing::get, Router};
use dms_coordinates::DMS;
use isocountry::CountryCode;
use regex::Regex;
use s2::{cell::Cell, cellid::CellID};

use super::AppError;

pub fn get_routes() -> Router {
  Router::new()
    .route("/21/coords/:binary", get(task_1))
    .route("/21/country/:binary", get(task_2))
}

async fn task_1(Path(binary): Path<String>) -> Result<String, AppError> {
  let tmp = u64::from_str_radix(&binary, 2)?;

  let center = Cell::from(CellID(tmp)).center();
  let lat = DMS::from_decimal_degrees(center.latitude().deg(), true);
  let lon = DMS::from_decimal_degrees(center.longitude().deg(), false);

  let lat = my_tostring(lat);
  let lon = my_tostring(lon);

  Ok(format!("{lat} {lon}"))
}

fn my_tostring(dms: DMS) -> String {
  format!(
    "{}°{}'{:.3}''{}",
    dms.degrees, dms.minutes, dms.seconds, dms.bearing
  )
}

async fn task_2(Path(binary): Path<String>) -> Result<String, AppError> {
  let tmp = u64::from_str_radix(&binary, 2)?;

  let center = Cell::from(CellID(tmp)).center();
  let lat = center.latitude();
  let lon = center.longitude();

  let url = format!("https://nominatim.openstreetmap.org/reverse?lat={lat:?}&lon={lon:?}");

  let res = ureq::get(&url).call()?.into_string()?;

  let re = Regex::new(r".*<country_code>(.+)</country_code>.*")?;

  let (_, [country_code]): (&str, [&str; 1]) = re
    .captures(&res)
    .ok_or_else(|| anyhow!("No country in response"))?
    .extract();

  let country_code = country_code.to_ascii_uppercase();

  let country = CountryCode::for_alpha2(&country_code)?.name();

  if country == "Brunei Darussalam" {
    return Ok("Brunei".to_string());
  }

  Ok(country.to_string())
}
