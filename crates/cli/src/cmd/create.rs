use std::{collections::HashMap, fs, path::Path};

use inquire::Confirm;
use proplate_core::{
  gen::bootstrap::bootstrap,
  template::{config::TemplateConf, inquirer::Input, resolver::clone_template, Template},
};
use proplate_errors::{CliErrorKind, ProplateError, ProplateErrorKind, ProplateResult};
use proplate_integration::git;
use proplate_tui::logger;

#[derive(Debug, Default)]
pub struct CreateOptions {
  pub git: bool,
}

type Context = HashMap<String, String>;

/// Create project starter
/// entrypoint for cli since it has lot more interaction:D
pub fn create(source: &str, dest: &str, options: CreateOptions) -> ProplateResult<()> {
  println!("{}", logger::title("Setup template"));
  let mut fork = fork_template(source, dest)?;
  let ctx = prompt_args(&fork)?;

  if options.git {
    init_git_repo(&fork.base_path)?
  }

  bootstrap(&mut fork, dest, &ctx)?;

  Ok(())
}

/// Create copy of a template in a tempdir
fn fork_template(from: &str, dest: &str) -> ProplateResult<Template> {
  println!("{}", logger::step("Finding template..."));
  clone_template(from, dest)
}

/// Interactively prompts args under "meta.args"
fn prompt_args(template: &Template) -> ProplateResult<Context> {
  let mut ctx = Context::new();
  let TemplateConf { args, .. } = &template.conf;

  println!("{}", logger::title("Template initialization:"));

  for arg in args {
    let input = Input::from(arg);
    ctx.insert(input.get_attr().name.clone(), input.prompt());
  }

  Ok(ctx)
}

fn init_git_repo(path: &Path) -> ProplateResult<()> {
  let lockfile = path.join(".git");

  if lockfile.exists() {
    let reinitialize =
      Confirm::new("Git is already initialized in the template, Do you want to reinitialize ?")
        .prompt()
        .map_err(|_| {
          ProplateError::create(ProplateErrorKind::Cli(CliErrorKind::Prompt))
            .with_ctx("cli::create::git_repo")
            .with_cause("Expected a user interaction")
        })?;
    if !reinitialize {
      return Ok(());
    }
    fs::remove_dir_all(&lockfile).map_err(|_| {
      ProplateError::create(ProplateErrorKind::Fs {
        concerned_paths: vec![lockfile.display().to_string()],
        operation: "remove_dir_all".into(),
      })
      .with_ctx("cli::create::git_repo")
      .with_cause("Unable to remove lockfile")
    })?;
  }

  do_init_git_repo(path)?;

  Ok(())
}

fn do_init_git_repo(path: &Path) -> ProplateResult<()> {
  println!("{}", logger::title("Initializing git repo"));
  git::exec_cmd(["init"], path)?;
  git::exec_cmd(["add", "-A"], path)?;
  git::exec_cmd(
    ["commit", "-m", "chore: initial commit", "--allow-empty"],
    path,
  )?;
  Ok(())
}
