use std::env::current_dir;
use std::fs::read_dir;
use std::io::Error;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::errors::{ProplateError, ProplateResult};
use crate::wrapper::exec_git_cmd;

use super::Template;

const BUILT_IN_TEMPLATE_DIR: &str = "built_in";

pub fn find_template(location: &str) -> ProplateResult<Template> {
    let is_remote = is_remote_loc(location);
    match is_remote {
        true => clone_git_template(location),
        false => find_builtin_template_by_id(location),
    }
}

fn find_builtin_template_by_id(id: &str) -> ProplateResult<Template> {
    let path = get_template_path_by_id(id);
    if !path.exists() {
        return Err(ProplateError::local_template_not_found(id));
    }
    explore_meta(path.try_into().unwrap(), &id, None)
        .map_err(|_| ProplateError::local_template_not_found(id))
}

fn clone_git_template(url: &str) -> ProplateResult<Template> {
    let path = url.strip_prefix("https://github.com/").unwrap();
    let id = path.split("/").collect::<Vec<_>>().join("-");
    let path = format!(".temp/{}-{}", id, Uuid::new_v4());
    exec_git_cmd(["clone", url, &path], &current_dir().unwrap())
        .map_err(|_| ProplateError::remote_template_not_found(url))?;
    explore_meta(path.try_into().unwrap(), &id, Some(url.to_string()))
        .map_err(|_| ProplateError::remote_template_not_found(url))
}

fn explore_meta(path: PathBuf, id: &str, source: Option<String>) -> Result<Template, Error> {
    let entries = read_dir(&path)?;
    let file_list = entries
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry.file_name()),
            _ => None,
        })
        .collect::<Vec<_>>();
    Ok(Template::build(id.to_string(), path, file_list, source))
}

fn is_remote_loc(location: &str) -> bool {
    location.starts_with("https://github.com/")
}

fn get_template_path_by_id(id: &str) -> PathBuf {
    Path::new(BUILT_IN_TEMPLATE_DIR).join(Path::new(id))
}
