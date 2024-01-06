use std::{
  fs,
  io::{Error, ErrorKind},
  path::{Path, PathBuf},
};

/// # Example
///
/// Let's walk this path: "C:\Users\proplate\samples\exclude_files\ban-node-modules"
///
/// For the file ".proplate_aux_utils\.gitkeep", WalkdirPathBuf would look like
///
/// ```
/// (
///  r#"C:\Users\proplate\samples\exclude_files\ban-node-modules\.proplate_aux_utils\.gitkeep"#,
///  ".proplate_aux_utils\.gitkeep"
/// )
/// ```
pub type WalkdirPathBuf = (/*abs*/ PathBuf, /*relative*/ PathBuf);

struct WalkDir {
  skip: Vec<PathBuf>,
}

impl WalkDir {
  pub fn new(skip: Option<Vec<PathBuf>>) -> WalkDir {
    WalkDir {
      skip: skip.unwrap_or_else(|| Vec::new()),
    }
  }

  pub fn walk(&self, path: &Path, dir: Option<PathBuf>) -> std::io::Result<Vec<WalkdirPathBuf>> {
    if path.is_file() {
      // TODO: Avoid alloc vec for no use
      return Ok(vec![self.walk_file(path, &dir.unwrap_or("".into()))]);
    }
    let mut dirs: Vec<WalkdirPathBuf> = Vec::new();
    self.walk_dir(&path, &dir.unwrap_or("".into()), &mut dirs)?;

    Ok(dirs)
  }

  fn walk_file(&self, path: &Path, dir: &Path) -> WalkdirPathBuf {
    (path.to_owned(), dir.to_owned())
  }

  fn walk_dir(
    &self,
    path: &Path,
    dir: &Path,
    dirs: &mut Vec<WalkdirPathBuf>,
  ) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();

      if self.skip.contains(&path) {
        continue;
      }

      let file_name = path.file_name().ok_or_else(|| {
        Error::new(
          ErrorKind::InvalidInput,
          "File does not have a valid filename",
        )
      })?;
      let dir = dir.join(file_name);
      dirs.extend(self.walk(&path, Some(dir))?);
    }
    Ok(())
  }
}

pub fn walk_dir_skip(path: &Path, skip: Vec<PathBuf>) -> std::io::Result<Vec<WalkdirPathBuf>> {
  let wd = WalkDir::new(Some(skip));
  let paths = wd.walk(path, None /*dir*/)?;
  Ok(paths)
}

pub fn walk_dir(path: &Path) -> std::io::Result<Vec<WalkdirPathBuf>> {
  let wd = WalkDir::new(None /*skip*/);
  let paths = wd.walk(path, None /*dir*/)?;
  Ok(paths)
}
