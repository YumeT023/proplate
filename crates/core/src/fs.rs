use std::{
  fs,
  io::{self, Error, ErrorKind},
  path::{Path, PathBuf},
};

pub fn copy_directory(src: &Path, dst: &Path) -> Result<(), Error> {
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let path = entry.path();
    let file_name = path.file_name().ok_or_else(|| {
      Error::new(
        ErrorKind::InvalidInput,
        "File does not have a valid filename",
      )
    })?;
    let dst_path = dst.join(file_name);
    if path.is_dir() {
      fs::create_dir(&dst_path)?;
      copy_directory(&path, &dst_path)?;
    } else {
      fs::copy(&path, &dst_path)?;
    }
  }
  Ok(())
}

/// Updates the provided file content
pub fn map_file(path: &Path, f: impl Fn(&str) -> String) -> Result<(), Error> {
  let content = fs::read_to_string(path)?;
  fs::write(path, f(&content))
}

pub fn canonic_path_from_str_vec(strings: Vec<String>) -> io::Result<std::path::PathBuf> {
  strings
    .iter()
    .map(|e| PathBuf::from(e))
    .reduce(|acc, p| acc.join(p))
    .expect("join str as path")
    .canonicalize()
}
