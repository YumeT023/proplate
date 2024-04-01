use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use proplate_tui::logger;
use uuid::Uuid;

use proplate_errors::{ProplateError, ProplateErrorKind, ProplateResult, TemplateErrorKind};
use proplate_integration::git;

use crate::join_path;

use crate::{fs as pfs, template::Template};

/// Attemps to find a template at the given location
/// It can be either a local path or a github repo url
pub fn clone_template(location: &str, dest: &str) -> ProplateResult<Template> {
  if !is_valid_location(location) {
    return Err(
      ProplateError::create(ProplateErrorKind::Template {
        kind: TemplateErrorKind::NotFound { is_remote: false },
        location: location.into(),
      })
      .with_ctx("template:clone")
      .with_cause("The location is neither a local nor a git repo"),
    );
  }
  match is_remote_uri(location) {
    true => clone_remote_template(location),
    false => clone_local_template(location, dest),
  }
}

fn clone_local_template(location: &str, dest: &str) -> ProplateResult<Template> {
  // make unique id
  let path = join_path!(".temp", format!("{}-{}", dest, Uuid::new_v4()));
  let from = Path::new(location);

  println!(
    "{}",
    logger::step(&format!("Cloning local template {}...", location))
  );

  pfs::copy_fdir(from, &path, None).map_err(|e| {
    ProplateError::create(ProplateErrorKind::Fs {
      concerned_paths: vec![from.display().to_string(), path.display().to_string()],
      operation: "copy_fdir".into(),
    })
    .with_ctx("template:local:clone")
    .with_cause(&e.to_string())
  })?;

  template_with_filebase(path.into(), location, location.into())
}

fn clone_remote_template(uri: &str) -> ProplateResult<Template> {
  let tail = uri.strip_prefix("https://github.com/").unwrap();

  // make unique id
  let id = tail.split("/").collect::<Vec<_>>().join("-");
  let dest = join_path!(".temp", format!("{}-{}", id, Uuid::new_v4()));

  println!(
    "{}",
    logger::step(&format!("Cloning template from git repo {}...", uri))
  );

  // TODO: shouldn't be done here
  git::exec_cmd(
    ["clone", uri, dest.to_str().unwrap()],
    &current_dir().unwrap(),
  )
  .map_err(|_| {
    ProplateError::create(ProplateErrorKind::Template {
      kind: TemplateErrorKind::NotFound { is_remote: true },
      location: uri.into(),
    })
    .with_ctx("template:remote:clone")
    .with_cause("git clone failed")
  })?;

  template_with_filebase(dest, &id, uri.to_string())
}

// TODO: move to Template struct
/// Create a template representation based on the provided meta
fn template_with_filebase(path: PathBuf, id: &str, source: String) -> ProplateResult<Template> {
  let file_list = fs::read_dir(&path)
    .map_err(|e| {
      ProplateError::create(ProplateErrorKind::Fs {
        concerned_paths: vec![path.display().to_string()],
        operation: "read_dir".into(),
      })
      .with_ctx("template:create")
      .with_cause(&e.to_string())
    })?
    .into_iter()
    .filter_map(|e| match e {
      Ok(entry) => entry.file_name().to_str().map(|s| s.to_string()).or(None),
      _ => None,
    })
    .collect::<Vec<_>>();
  Ok(Template::build(id.to_string(), path, file_list, source))
}

fn is_remote_uri(uri: &str) -> bool {
  uri.starts_with("https://github.com/")
}

fn is_local_dir(uri: &str) -> bool {
  let path = PathBuf::from(uri);
  path.exists() && path.is_dir()
}

fn is_valid_location(location: &str) -> bool {
  is_remote_uri(location) || is_local_dir(location)
}
