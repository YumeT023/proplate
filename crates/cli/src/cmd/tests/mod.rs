use std::{
  fs,
  path::{Path, PathBuf},
  process::Command,
};

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

fn get_sample(name: &str) -> PathBuf {
  get_path("samples").join(name)
}

fn get_snapshot(name: &str) -> PathBuf {
  get_path("samples/snapshot").join(name)
}

/// New temporary dir (calling it trash cuz... !!)
fn new_trash() -> (PathBuf, String /*uuid*/) {
  let uuid = Uuid::new_v4().to_string();
  (get_trash().join(&uuid), uuid)
}

/// because I have to clean it here...
fn cleanup_test_trash() {
  fs::remove_dir_all(get_trash()).expect("rm test_trash");
}

fn run_isolated_test(f: impl Fn() -> ProplateResult<()>, clean: bool) {
  match f() {
    Err(e) => {
      panic!("{}: {}:{}", e.print_err(), line!(), column!())
    }
    _ => (),
  }
  clean.then(|| cleanup_test_trash());
}

#[macro_export]
macro_rules! assert_gen_snapshot {
    ($snapshot: expr, $generated: expr, $($files: expr)+) => {
      $({
        let snap = fs::read_to_string($snapshot.join($files)).expect("read snap");
        let gen = fs::read_to_string($generated.join($files)).expect("read gen");
        assert_eq!( snap, gen );
      })+;
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
