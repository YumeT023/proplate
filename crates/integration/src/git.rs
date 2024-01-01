use std::{
  path::Path,
  process::{Command, Stdio},
};

use proplate_errors::{ProplateError, ProplateResult};
use proplate_tui::logger;

pub fn exec_cmd<'a>(cmd: impl IntoIterator<Item = &'a str>, path: &Path) -> ProplateResult<()> {
  let child = Command::new("git")
    .args(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(path)
    .spawn()
    .map_err(|e| ProplateError::git(&e.to_string()))?;

  let output = child
    .wait_with_output()
    .map_err(|e| ProplateError::git(&e.to_string()))?;

  match output.status.success() {
    true => {
      if !output.stdout.is_empty() {
        println!(
          "{}",
          logger::success(&String::from_utf8_lossy(&output.stdout))
        );
      }
      Ok(())
    }
    false => Err(ProplateError::git(&String::from_utf8_lossy(&output.stderr))),
  }
}
