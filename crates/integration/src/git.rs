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
  let subcmd = cmd.into_iter().next().unwrap();

  let child = Command::new("git")
    .args(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(path)
    .spawn()
    .map_err(|e| {
      ProplateError::create(ProplateErrorKind::Git {
        cmd: subcmd.into(),
        raw_stderr: e.to_string(),
      })
    })?;

  let output = child.wait_with_output().map_err(|e| {
    ProplateError::create(ProplateErrorKind::Git {
      cmd: subcmd.into(),
      raw_stderr: e.to_string(),
    })
  })?;

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
    _ => Err(ProplateError::create(ProplateErrorKind::Git {
      cmd: subcmd.into(),
      raw_stderr: String::from_utf8_lossy(&output.stderr).into(),
    })),
  }
}
