mod common;

#[test]
fn check_cli() {
  let mut cmd = common::proplate_cli();
  cmd
    .arg("--version")
    .assert()
    .stdout(format!("proplate {}\n", env!("CARGO_PKG_VERSION")));
}
