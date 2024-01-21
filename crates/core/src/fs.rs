use std::{
  fs,
  path::{Path, PathBuf},
};

use self::walk::{walk_dir, walk_dir_skip};

pub mod walk;

// Recursively copies dir entries to another
pub fn copy_fdir(entry: &Path, dest: &Path, except: Option<Vec<PathBuf>>) -> std::io::Result<()> {
  fs::create_dir_all(dest)?;
  for (file, filename) in walk_dir_skip(entry, except.unwrap_or_default())? {
    let to = dest.join(&filename);
    if let Some(parent) = to.parent() {
      fs::create_dir_all(&parent)?;
    }
    fs::copy(&file, &to).expect(format!("copy: {}\n{}\n\n", file.display(), to.display()).as_str());
  }
  Ok(())
}

pub fn map_fdir(path: &Path, f: impl Fn(&str) -> String) -> std::io::Result<()> {
  for (file, _) in walk_dir(path)? {
    let content = fs::read_to_string(&file)?;
    fs::write(&file, f(content.as_str()))?;
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

pub fn is_dir_superset(dir1: &Path, dir2: &Path) -> std::io::Result<bool> {
  for (file, relative) in walk_dir(dir1)? {
    let a = fs::read_to_string(&file)?;
    let b = fs::read_to_string(dir2.join(&relative))?;
    if a != b {
      return Ok(false);
    }
  }
  Ok(true)
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
