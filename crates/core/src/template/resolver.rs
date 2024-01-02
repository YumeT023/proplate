use std::env::current_dir;
use std::fs::read_dir;
use std::io::Error;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use proplate_errors::{ProplateError, ProplateResult};
use proplate_integration::git;

use super::Template;

/// Attemps to find a template at the given location
/// It can be either a local path or a github repo url
pub fn find_template(location: &str) -> ProplateResult<Template> {
  match is_remote_loc(location) {
    true => clone_git_template(location),
    false => find_local_template(location),
  }
}

fn find_local_template(dir: &str) -> ProplateResult<Template> {
  let path = Path::new(dir);
  if !path.exists() {
    return Err(ProplateError::local_template_not_found(dir));
  }
  explore_meta(path.try_into().unwrap(), &dir, None)
    .map_err(|_| ProplateError::local_template_not_found(dir))
}

fn clone_git_template(url: &str) -> ProplateResult<Template> {
  let path = url.strip_prefix("https://github.com/").unwrap();
  // make unique id
  let id = path.split("/").collect::<Vec<_>>().join("-");
  let path = format!(".temp/{}-{}", id, Uuid::new_v4());

  git::exec_cmd(["clone", url, &path], &current_dir().unwrap())
    .map_err(|_| ProplateError::remote_template_not_found(url))?;

  explore_meta(path.try_into().unwrap(), &id, Some(url.to_string()))
    .map_err(|e| ProplateError::fs(&e.to_string()))
}

// TODO: move to Template struct
/// Create a template representation based on the provided meta
fn explore_meta(path: PathBuf, id: &str, source: Option<String>) -> Result<Template, Error> {
  let file_list = read_dir(&path)?
    .into_iter()
    .filter_map(|e| match e {
      Ok(entry) => entry.file_name().to_str().map(|s| s.to_string()).or(None),
      _ => None,
    })
    .collect::<Vec<_>>();
  Ok(Template::build(id.to_string(), path, file_list, source))
}

fn is_remote_loc(location: &str) -> bool {
  location.starts_with("https://github.com/")
}
