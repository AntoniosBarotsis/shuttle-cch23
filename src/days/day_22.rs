use anyhow::anyhow;
use axum::{routing::post, Router};
use pathfinding::directed::bfs::bfs;

use super::AppError;

pub fn get_routes() -> Router {
  Router::new()
    .route("/22/integers", post(task_1))
    .route("/22/rocket", post(task_2))
}

async fn task_1(payload: String) -> Result<String, AppError> {
  let lines = payload
    .lines()
    .map(|el| str::parse::<u64>(el).expect("Parse"))
    .collect::<Vec<_>>();

  let mut carry = 0;
  for num in lines {
    carry ^= num;
  }

  Ok("🎁".repeat(usize::try_from(carry)?))
}

#[allow(clippy::unwrap_used)]
async fn task_2(payload: String) -> Result<String, AppError> {
  // If you are reading this, probably not the cleanest idea to use a whole ahh crate for this
  // I just wanted to check out how it works, I saw it a while back and never tried it out :)
  let lines = payload.lines().collect::<Vec<_>>();

  let number_of_stars = str::parse::<usize>(lines[0])?;
  let stars = &lines[1..=number_of_stars];
  let number_of_portals = str::parse::<usize>(lines[number_of_stars + 1])?;
  let portals = &lines[number_of_stars + 2..number_of_stars + 2 + number_of_portals];

  let points = stars
    .iter()
    .map(|l| l.split(' '))
    .map(|mut nums| {
      (
        nums.next().unwrap(),
        nums.next().unwrap(),
        nums.next().unwrap(),
      )
    })
    .map(|(x, y, z)| {
      (
        str::parse::<i32>(x).unwrap(),
        str::parse::<i32>(y).unwrap(),
        str::parse::<i32>(z).unwrap(),
      )
    })
    .collect::<Vec<_>>();

  let point_indices = (0..number_of_stars).map(Point).collect::<Vec<_>>();

  let edges = portals
    .iter()
    .map(|s| s.split(' ').map(|n| str::parse::<usize>(n).expect("parse")))
    .map(|mut nums| Edge(nums.next().unwrap(), nums.next().unwrap()))
    .collect::<Vec<_>>();

  let result = bfs(
    &point_indices[0],
    |p| p.successors(&point_indices, &edges),
    |p| *p == *point_indices.last().unwrap(),
  )
  .ok_or_else(|| anyhow!("Path not found"))?;

  let points_in_path = result.iter().map(|Point(i)| points[*i]).collect::<Vec<_>>();

  let mut dist = 0f32;
  for window in points_in_path.windows(2) {
    let p1 = window[0];
    let p2 = window[1];

    dist += distance(&p1, &p2);
  }

  let len = points_in_path.len() - 1;

  Ok(format!("{len} {dist:.3}"))
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(usize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Edge(usize, usize);

impl Point {
  fn successors(&self, points: &[Self], edges: &[Edge]) -> Vec<Self> {
    let &Self(i) = self;

    let edges = edges
      .iter()
      .filter(|e| e.0 == i)
      .map(|e| e.1)
      .collect::<Vec<_>>();
    let points = edges.iter().map(|e| points[*e].clone()).collect::<Vec<_>>();

    points
  }
}

fn distance(p1: &(i32, i32, i32), p2: &(i32, i32, i32)) -> f32 {
  let tmp = (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2) + (p2.2 - p1.2).pow(2);

  #[allow(clippy::cast_precision_loss)]
  (tmp as f32).sqrt()
}
