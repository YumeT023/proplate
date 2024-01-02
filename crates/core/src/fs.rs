use std::{
  fs,
  io::{Error, ErrorKind},
  path::Path,
};

pub fn copy_directory(src: &Path, dest: &Path) -> std::io::Result<()> {
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
    if path.is_dir() {
      fs::create_dir(&dest_path)?;
      copy_directory(&path, &dest_path)?;
    } else {
      fs::copy(&path, &dest_path)?;
    }
  }
  Ok(())
}

pub fn copy_file(src: &Path, dest: &Path) -> std::io::Result<()> {
}

/// Updates the provided file content
pub fn map_file(path: &Path, f: impl Fn(&str) -> String) -> Result<(), Error> {
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
