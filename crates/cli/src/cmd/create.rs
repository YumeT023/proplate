use std::{collections::HashMap, fs, path::Path};

use inquire::Confirm;
use proplate_core::{
  fs as pfs,
  template::{
    config::{analyze_dyn_files, TemplateConf},
    inquirer::Input,
    interpolation::MapWithCtx,
    op::Execute,
    resolver::clone_template,
    Template,
  },
};
use proplate_errors::{ProplateError, ProplateResult};
use proplate_integration::git;
use proplate_tui::logger;

#[derive(Debug, Default)]
pub struct CreateOptions {
  pub git: bool,
}

type Context = HashMap<String, String>;

/// Create project starter
/// entrypoint for cli since it has lot more interaction:D
pub fn run_create(source: &str, dest: &str, options: CreateOptions) -> ProplateResult<()> {
  println!("{}", logger::title("Setup template"));
  let mut fork = fork_template(source, dest)?;
  let ctx = prompt_args(&fork)?;

  if options.git {
    init_git_repo(&fork.base_path)?
  }

  create(&mut fork, dest, options, &ctx)?;
  Ok(())
}

/// Create project starter
/// lib entrypoint
pub fn create(
  fork: &mut Template,
  dest: &str,
  _options: CreateOptions,
  ctx: &Context,
) -> ProplateResult<()> {
  process_template(fork, ctx)?;
  prepare_dest(dest)?;
  copy_files(fork, dest)?;
  cleanup(fork)?;
  Ok(())
}

/// Create project dest dir
fn prepare_dest(dest: &str) -> ProplateResult<()> {
  println!("{}", logger::title("Finalizing"));
  fs::create_dir_all(dest)
    .map_err(|e| ProplateError::fs(&format!("{}", e.to_string()), vec![Path::new(&dest)]))?;
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

/// Executes hook and bind ctx onto dynamic_files.
fn process_template(template: &mut Template, ctx: &Context) -> ProplateResult<()> {
  println!("{}", logger::step("Running additional operations..."));

  // run "additional_operations" in order to process the dynamically
  // added file in the extra operation.
  for op in &template.conf.additional_operations {
    op.execute(&ctx)?;
  }

  println!(
    "{}",
    logger::step("Verifying whether analysis of dyn files is necessary...")
  );
  if template.conf.require_dyn_file_analysis {
    analyze_dyn_files(&mut template.conf, &template.base_path);
  }

  println!("{}", logger::step("Binding ctx to dynamic_files..."));

  for filepath in &template.conf.dynamic_files {
    println!("      {}", logger::step(&format!("processing...")));
    bind_ctx_to_file(Path::new(&filepath), ctx);
  }

  Ok(())
}

/// Replaces dynamic var "$var" with their actual value
fn bind_ctx_to_file(path: &Path, ctx: &Context) {
  match pfs::map_file(path, |s| s.to_string().map_with_ctx(ctx)) {
    Err(_) => {
      // TODO: warn if not found but wasn't removed in additional_op either
    }
    _ => (),
  }
}

/// Copies template file to the provided dest
/// Files under "meta.exclude" won't be copied
fn copy_files(template: &Template, dest: &str) -> ProplateResult<()> {
  let src = &template.base_path;
  let dest = Path::new(dest);

  println!("{}", logger::step("Copying..."));

  pfs::copy_fdir(
    src,
    dest,
    Some(
      template
        .conf
        .exclude
        .iter()
        .map(|s| s.into())
        .collect::<Vec<_>>(),
    ),
  )
  .map_err(|e| {
    ProplateError::fs(
      &format!("{}", e.to_string()),
      vec![&template.base_path, Path::new(&dest)],
    )
  })
}

fn init_git_repo(path: &Path) -> ProplateResult<()> {
  let lockfile = path.join(".git");

  if lockfile.exists() {
    let reinitialize =
      Confirm::new("Git is already initialized in the template, Do you want to reinitialize ?")
        .prompt()
        .map_err(|e| ProplateError::prompt(&e.to_string()))?;
    if !reinitialize {
      return Ok(());
    }
    fs::remove_dir_all(&lockfile)
      .map_err(|e| ProplateError::fs(&e.to_string(), vec![&lockfile]))?;
  }

  _init_git_repo(path)?;

  Ok(())
}

fn _init_git_repo(path: &Path) -> ProplateResult<()> {
  println!("{}", logger::title("Initializing git repo"));
  git::exec_cmd(["init"], path)?;
  git::exec_cmd(["add", "-A"], path)?;
  git::exec_cmd(
    ["commit", "-m", "chore: initial commit", "--allow-empty"],
    path,
  )?;
  Ok(())
}

fn cleanup(fork: &Template) -> ProplateResult<()> {
  println!("{}", logger::step("cleaning up..."));
  fs::remove_dir_all(&fork.base_path)
    .map_err(|_| ProplateError::fs("unable to cleanup tmp...", vec![&fork.base_path]))?;
  Ok(())
}
