use std::{
  path::Path,
  process::{Command, Stdio},
};

use proplate_errors::{ProplateError, ProplateErrorKind, ProplateResult};
use proplate_tui::logger;

pub fn exec_cmd<'a, I: IntoIterator<Item = &'a str> + Copy>(
  cmd: I,
  path: &Path,
) -> ProplateResult<()> {
  let child = Command::new("git")
    .args(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(path)
    .spawn()
    .map_err(|e| ProplateError::create(ProplateErrorKind::git("".into(), e.to_string())))?;

  let cmd = cmd.into_iter().next().unwrap();

  let output = child
    .wait_with_output()
    .map_err(|e| ProplateError::create(ProplateErrorKind::git(cmd.into(), e.to_string())))?;

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
    false => Err(ProplateError::create(ProplateErrorKind::git(
      "".into(),
      String::from_utf8_lossy(&output.stderr).into(),
    ))),
  }
}
