use axum::{routing::post, Json, Router};

pub fn get_routes() -> Router {
  Router::new()
    .route("/14/unsafe", post(unsafe_render))
    .route("/14/safe", post(safe_render))
  // .route("/13/orders", post(insert))
  // .route("/13/orders/total", get(total))
  // .route("/13/orders/popular", get(popular))
}

#[derive(serde::Deserialize, Debug)]
struct SimpleBody {
  content: String,
}

fn html_boilerplate(html: &str) -> String {
  format!(
    r"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {html}
  </body>
</html>"
  )
}

async fn unsafe_render(Json(payload): Json<SimpleBody>) -> String {
  html_boilerplate(&payload.content)
}

async fn safe_render(Json(payload): Json<SimpleBody>) -> String {
  let unsanitized = payload.content;

  let sanitized = unsanitized
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;");

  html_boilerplate(&sanitized)
}
