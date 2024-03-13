use std::{
  env::current_exe,
  path::{Path, PathBuf},
};

use proplate_errors::{ProplateError, ProplateResult};

pub fn local_template_path() -> PathBuf {
  proplate_dir().join("builtins").join("templates")
}

pub fn get_local_template<P>(path: P) -> ProplateResult<PathBuf>
where
  P: AsRef<Path>,
{
  let tpath = local_template_path().join(path);
  match tpath.exists() {
        true => Ok(tpath),
        _ => Err(ProplateError::local_template_not_found(tpath.display().to_string().as_str()))
  }
}

pub fn proplate_dir() -> PathBuf {
  let exe = current_exe().expect("Unable to resolve proplate path");
  exe.parent().unwrap().to_owned()
}
