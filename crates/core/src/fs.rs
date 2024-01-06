use std::{
  fs,
  path::{Path, PathBuf},
};

use self::walk::walk_dir_skip;

mod walk;

/// Copies file/dir recursively
pub fn copy_fdir(src: &Path, dest: &Path, except: Option<Vec<PathBuf>>) -> std::io::Result<()> {
  fs::create_dir_all(dest)?;
  for (file, filename) in walk_dir_skip(src, except.unwrap_or_default())? {
    let to = dest.join(filename);
    if let Some(parent) = to.parent() {
      fs::create_dir_all(&parent)?;
    }
    fs::copy(file.clone(), &to)?;
  }
  Ok(())
}

// Remove file/dir recursively
pub fn remove_fdir(path: &Path) -> std::io::Result<()> {
  if !path.exists() {
    return Ok(());
  }

  if path.is_file() {
    fs::remove_file(path)?
  } else {
    fs::remove_dir_all(path)?
  }

  Ok(())
}

/// Updates the provided file content
pub fn map_file(path: &Path, f: impl Fn(&str) -> String) -> std::io::Result<()> {
  let content = fs::read_to_string(path)?;
  fs::write(path, f(&content))
}

#[macro_export]
macro_rules! join_path {
  ($($path:expr),*) => ({
    let mut buf = PathBuf::new();
    $(
      buf.push($path);
    )*
    buf.canonicalize().unwrap_or(buf)
  })
}
