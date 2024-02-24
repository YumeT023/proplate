#[cfg(test)]
mod gen_test;

use std::{
  fs,
  path::{Path, PathBuf},
  process::Command,
};

use crate::{fs::walk::walk_dir, join_path};
use proplate_errors::ProplateResult;
use proplate_tui::logger::AsError;
use uuid::Uuid;

fn workspace_dir() -> PathBuf {
  let output = Command::new(env!("CARGO"))
    .arg("locate-project")
    .arg("--workspace")
    .arg("--message-format=plain")
    .output()
    .unwrap()
    .stdout;
  let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
  cargo_path.parent().unwrap().to_path_buf()
}

fn get_path(path: &str) -> PathBuf {
  workspace_dir().join(path)
}

fn get_trash() -> PathBuf {
  get_path("test_trash")
}

fn get_sample(pkg: &str, name: &str) -> (PathBuf, PathBuf /*snapshot*/) {
  let path = get_path(
    join_path!("samples", pkg, name)
      .display()
      .to_string()
      .as_str(),
  );
  (
    path.clone(),
    (path.display().to_string() + "-snapshot").into(),
  )
}

/// New temporary dir (calling it trash cuz... !!)
fn new_trash() -> (PathBuf, String /*uuid*/) {
  let uuid = Uuid::new_v4().to_string();
  (get_trash().join(&uuid), uuid)
}

/// because I have to clean it here...
#[allow(unused)]
fn cleanup_test_trash() {
  fs::remove_dir_all(get_trash()).expect("rm test_trash");
}

fn run_isolated_test(f: impl Fn() -> ProplateResult<()>, _clean: bool) {
  match f() {
    Err(e) => {
      panic!("{}: {}:{}", e.print_err(), line!(), column!())
    }
    _ => (),
  }
  // rust tests run in parallel
  // fixme: sadly.... WE CANNOT CLEANUP THE TRASH... as some tests may still use it
  // clean.then(|| cleanup_test_trash());
}

// TODO: some op are weird
fn assert_dir_superset(dir1: &Path, dir2: &Path) -> std::io::Result<()> {
  for (file, relative) in walk_dir(dir1)? {
    let a = fs::read_to_string(&file).expect(format!("Fail {}", file.display()).as_str());
    let b = fs::read_to_string(dir2.join(&relative))
      .expect(format!("Fail {}", dir2.join(&relative).display()).as_str());
    assert_eq!(a, b);
  }
  Ok(())
}

#[macro_export]
macro_rules! assert_gen_snapshot {
  ($snapshot: expr, $generated: expr) => {
    assert_dir_superset(&$snapshot, &$generated).expect("test 1");
    assert_dir_superset(&$generated, &$snapshot).expect("test 2");
  };
}

/// Ensures the following
/// - The project is generated
/// - The generated has neither "meta.json" or ".proplate_aux_utils"
#[macro_export]
macro_rules! assert_gen_ok {
  ($path: expr) => {
    assert!($path.exists());
    assert!(!$path.join(META_CONF).exists());
    assert!(!$path.join(".proplate_aux_utils").exists());
  };
}

/// Clones and creates template instance in trash_dir
/// The reason it is a test utility is that, I suppose, you could not possibly know in advance the ctx that the template needs.
#[macro_export]
macro_rules! test_create {
  ($pkg: expr, $name: expr, $ctx: expr) => {{
    let (path, _uuid) = new_trash();
    let dest = path.display().to_string();

    let (t, snap) = get_sample($pkg, $name);

    let mut fork = clone_template(t.display().to_string().as_str(), &dest)?;
    bootstrap(&mut fork, &dest, &$ctx)?;

    (path, snap)
  }};
}
