use axum::{extract::Path, routing::get, Router};

pub fn get_routes() -> Router {
  Router::new()
    // .route("/1/:num1/:num2", get(task_1))
    .route("/1/*nums", get(task_2))
}

#[allow(clippy::unused_async, dead_code)]
async fn task_1(Path((num1, num2)): Path<(i32, i32)>) -> String {
  (num1 ^ num2).pow(3).to_string()
}

async fn task_2(Path(args): Path<String>) -> String {
  let res = args
    .split('/')
    .map(|el| el.parse::<i32>().expect("Path arg is number"))
    .reduce(|acc, e| acc ^ e)
    .expect("Iterator not empty")
    .pow(3);

  res.to_string()
}
