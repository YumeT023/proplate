use std::{
  env::current_exe,
  path::{Path, PathBuf},
  process::exit,
};

use proplate_tui::logger::error;

const NO_BUILTIN_DIR_ERROR_MSG: &str = r#"You havent installed 'proplate' through gh release, its missing 'builtin' dir which is used to initialize new template locally
   Instead, do `proplate create --template https://github.com/YumeT023/tiniest-proplate`"#;

pub fn local_template_path() -> PathBuf {
  proplate_dir().join("builtins").join("templates")
}

pub fn get_local_template<P>(path: P) -> PathBuf
where
  P: AsRef<Path>,
{
  let path = local_template_path().join(path);
  match path.exists() {
    true => path,
    _ => {
      eprintln!("{}", error(NO_BUILTIN_DIR_ERROR_MSG));
      exit(1);
    }
  }
}

pub fn proplate_dir() -> PathBuf {
  let exe = current_exe().expect("Unable to resolve proplate path");
  exe.parent().unwrap().to_owned()
}
