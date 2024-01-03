use std::{
  fs,
  io::{Error, ErrorKind},
  path::Path,
};

/// Copies file/dir recursively
pub fn copy_fdir(src: &Path, dest: &Path) -> std::io::Result<()> {
  if src.is_file() {
    // Create the destination directory if it doesn't exist
    if let Some(parent) = dest.parent() {
      fs::create_dir_all(&parent)?;
    }
    fs::copy(&src, &dest)?;
    return Ok(());
  }

  // Create the destination directory if it doesn't exist
  fs::create_dir_all(&dest)?;

  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let path = entry.path();
    let file_name = path.file_name().ok_or_else(|| {
      Error::new(
        ErrorKind::InvalidInput,
        "File does not have a valid filename",
      )
    })?;
    let dest_path = dest.join(file_name);
    copy_fdir(&path, &dest_path)?;
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
