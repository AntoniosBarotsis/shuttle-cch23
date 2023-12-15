use regex::Regex;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use unicode_segmentation::UnicodeSegmentation;

pub fn get_routes() -> Router {
  Router::new()
    .route("/15/nice", post(task_1))
    .route("/15/game", post(task_2))
}

#[derive(serde::Deserialize, Debug)]
struct SimpleBody {
  input: String,
}

async fn task_1(Json(payload): Json<SimpleBody>) -> impl IntoResponse {
  const VOWELS: &str = "aeiouy";
  let bad_strs: [&[u8]; 4] = [&[b'a', b'b'], &[b'c', b'd'], &[b'p', b'q'], &[b'x', b'y']];

  let input = &payload.input;

  let at_least_3_vowels = input.chars().filter(|c| VOWELS.contains(*c)).count() > 2;

  let letter_appears_twice_in_a_row = input
    .as_bytes()
    .windows(2)
    .filter(|el| el[0] == el[1] && el[0].is_ascii_alphabetic())
    .count()
    > 0;

  // ab, cd, pq, or xy
  let contains_bad_substr = input
    .as_bytes()
    .windows(2)
    .filter(|el| bad_strs.contains(el))
    .count()
    > 0;

  let is_nice = at_least_3_vowels && letter_appears_twice_in_a_row && !contains_bad_substr;

  if is_nice {
    (StatusCode::OK, "{\"result\":\"nice\"}".to_string())
  } else {
    (
      StatusCode::BAD_REQUEST,
      "{\"result\":\"naughty\"}".to_string(),
    )
  }
}

fn find_char_indices(input: &str, search: char) -> impl Iterator<Item = usize> + '_ {
  input
    .char_indices()
    .filter(move |(_i, c)| *c == search)
    .map(|el| el.0)
}

fn naughty_resp(msg: &str) -> String {
  format!("{{\"result\":\"naughty\",\"reason\":\"{msg}\"}}")
}

async fn task_2(Json(payload): Json<SimpleBody>) -> impl IntoResponse {
  let input = &payload.input;

  let rule_1 = input.len() > 7;

  let uppercase = input.chars().find(|c| c.is_uppercase());
  let lowercase = input.chars().find(|c| c.is_lowercase());
  let digit = input.chars().find(char::is_ascii_digit);

  let rule_2 = uppercase.and(lowercase).and(digit).is_some();

  let rule_3 = input.chars().filter(char::is_ascii_digit).count() > 4;

  let re = Regex::new(r"\d+").expect("compiles");
  let rule_4 = re
    .find_iter(input)
    .map(|el| str::parse::<u32>(el.as_str()).expect("Is a number"))
    .sum::<u32>()
    == 2023;

  let joy = find_char_indices(input, 'j')
    .chain(find_char_indices(input, 'o'))
    .chain(find_char_indices(input, 'y'))
    .collect::<Vec<_>>();
  let rule_5 = joy.len() == 3
    && joy
      .iter()
      .as_slice()
      .windows(2)
      .skip_while(|el| el[0] < el[1])
      .count()
      == 0;

  let rule_6 = input
    .as_bytes()
    .windows(3)
    .filter(|el| el[0] == el[2] && el[0].is_ascii_alphabetic())
    .count()
    > 0;

  let rule_7 = input
    .chars()
    .any(|c| (&'\u{2980}'..=&'\u{2BFF}').contains(&&c));

  let rule_8 = contains_emoji(input);

  let rule_9 = sha256::digest(input).ends_with('a');

  if !rule_1 {
    (StatusCode::BAD_REQUEST, naughty_resp("8 chars"))
  } else if !rule_2 {
    (StatusCode::BAD_REQUEST, naughty_resp("more types of chars"))
  } else if !rule_3 {
    (StatusCode::BAD_REQUEST, naughty_resp("55555"))
  } else if !rule_4 {
    (StatusCode::BAD_REQUEST, naughty_resp("math is hard"))
  } else if !rule_5 {
    (
      StatusCode::NOT_ACCEPTABLE,
      naughty_resp("not joyful enough"),
    )
  } else if !rule_6 {
    (
      StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
      naughty_resp("illegal: no sandwich"),
    )
  } else if !rule_7 {
    (StatusCode::RANGE_NOT_SATISFIABLE, naughty_resp("outranged"))
  } else if !rule_8 {
    (StatusCode::UPGRADE_REQUIRED, naughty_resp("😳"))
  } else if !rule_9 {
    (StatusCode::IM_A_TEAPOT, naughty_resp("not a coffee brewer"))
  } else {
    (
      StatusCode::OK,
      "{\"result\":\"nice\",\"reason\":\"that's a nice password\"}".to_string(),
    )
  }
}

pub fn contains_emoji(string: &str) -> bool {
  string.graphemes(true).any(|el| emojis::get(el).is_some())
}
