use assert_cmd::Command;

pub fn proplate_cli() -> Command {
  Command::cargo_bin("proplate").unwrap()
}
