use axum::{body::Bytes, routing::post, Router};
use git2::{BranchType, Repository, Signature};
use std::{
  fs::{self, File},
  io::{Read, Write},
};
use tar::{Archive, EntryType};
use tempfile::tempdir;
use tokio_util::bytes::Buf;
use tracing::{info, warn};

use super::AppError;

pub fn get_routes() -> Router {
  Router::new()
    .route("/20/archive_files", post(archive_count))
    .route("/20/archive_files_size", post(archive_size))
    .route("/20/cookie", post(cookie))
}

async fn archive_count(file: Bytes) -> Result<String, AppError> {
  let files = Archive::new(file.reader()).entries()?.count();

  Ok(files.to_string())
}

async fn archive_size(file: Bytes) -> Result<String, AppError> {
  let size = Archive::new(file.reader())
    .entries()?
    .map(|el| el.map(|f| f.size()).unwrap_or(0))
    .sum::<u64>();

  Ok(size.to_string())
}

#[allow(clippy::unwrap_used)]
async fn cookie(file: Bytes) -> Result<String, AppError> {
  const BRANCH_NAME: &str = "christmas";
  const FILE_NAME: &str = "santa.txt";

  let mut archive = Archive::new(file.reader());
  let temp_dir = tempdir()?;

  archive
    .entries()?
    .filter_map(std::result::Result::ok)
    .for_each(|mut el| {
      let file_path = temp_dir.path().join(el.path().unwrap());

      if matches!(el.header().entry_type(), EntryType::Directory) {
        fs::create_dir_all(file_path).unwrap();
      } else {
        let mut file = File::create(file_path).expect("Create file");

        let mut buffer = Vec::new();
        let _ = el.read_to_end(&mut buffer).unwrap();

        file.write_all(&buffer).expect("Write to file");
      }
    });

  let mut repo = Repository::open(temp_dir.path())?;
  let signature = Signature::now("Antonios Barotsis", "antonios.barotsis@protonmail.com")?;
  let _ = repo.stash_save(&signature, "stash", None)?;

  let branch = repo.find_branch(BRANCH_NAME, BranchType::Local)?;
  let mut commit = branch.get().peel_to_commit()?;

  let mut res = None::<String>;
  'outer: while let Ok(tree) = commit.tree() {
    let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;

    let paths = diff
      .deltas()
      .filter_map(|el| el.new_file().path())
      .filter(|path| path.to_str().unwrap().contains(FILE_NAME))
      .collect::<Vec<_>>();

    for path in paths {
      let full_path = temp_dir.path().join(path);

      info!("Checking out to {:?}", commit.id());
      repo.checkout_tree(commit.as_object(), None).unwrap();
      repo.set_head_detached(commit.id()).unwrap();

      if let Ok(file_contents) = fs::read_to_string(full_path) {
        if file_contents.contains("COOKIE") {
          let author = commit.author();
          let sha = commit.id();

          info!("{}, {:?}", author.name().unwrap(), sha);
          res = format!("{} {:?}", author.name().unwrap(), sha).into();
          break 'outer;
        }
      } else {
        warn!("File could not be read");
      }
    }

    if let Some(parent) = commit.parents().next() {
      commit = parent;
    } else {
      break;
    }
  }

  Ok(res.unwrap())
}
