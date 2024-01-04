use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
};

use inquire::Confirm;
use proplate_core::{
  fs as pfs, join_path,
  template::{
    config::TemplateConf, inquirer::Input, interpolation::MapWithCtx, op::Execute,
    resolver::find_template, Template,
  },
};
use proplate_errors::{ProplateError, ProplateResult};
use proplate_integration::git;
use proplate_tui::logger;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct CreateOptions {
  pub git: bool,
}

type Context = HashMap<String, String>;

/// Create project starter
/// Entrypoint for cli
pub fn create(source: &str, dest: &str, options: CreateOptions) -> ProplateResult<()> {
  let mut fork = fork_template(source, dest)?;
  let ctx = prompt_args(&fork)?;

  if options.git {
    init_git_repo(&fork.base_path)?
  }

  _create(&mut fork, dest, options, &ctx)?;
  Ok(())
}

/// Create project starter
/// impl details
fn _create(
  fork: &mut Template,
  dest: &str,
  _options: CreateOptions,
  ctx: &Context,
) -> ProplateResult<()> {
  normalize_template(fork);
  process_template(fork, ctx)?;
  prepare_dest(dest)?;
  copy_files(fork, dest)?;
  cleanup(fork)?;
  Ok(())
}

/// Create project dest dir
fn prepare_dest(dest: &str) -> ProplateResult<()> {
  fs::create_dir_all(dest)
    .map_err(|e| ProplateError::fs(&format!("{}", e.to_string()), vec![Path::new(&dest)]))?;
  Ok(())
}

/// Create copy of a template in a tempdir
fn fork_template(from: &str, dest: &str) -> ProplateResult<Template> {
  let mut template = find_template(from)?;

  // already cloned from 'github'
  if template.fork_source.is_some() {
    return Ok(template);
  }

  // copy temp local
  let forkpath = join_path!(".temp", format!("{}-{}", dest, Uuid::new_v4()));
  fs::create_dir_all(&forkpath).map_err(|e| {
    ProplateError::fs(
      &format!("{}", e.to_string()),
      vec![&forkpath, Path::new(&dest)],
    )
  })?;

  pfs::copy_fdir(&template.base_path, &forkpath, None).map_err(|e| {
    ProplateError::fs(
      &format!("{}", e.to_string()),
      vec![&template.base_path, &forkpath],
    )
  })?;

  // bind template mod to the temp path
  template.base_path = forkpath;
  Ok(template)
}

/// Canonicalizes paths under "meta.dynamic_files", "meta.additional_operations" and "meta.exclude"
fn normalize_template(template: &mut Template) {
  Template::normalize_template(template);
}

/// Interactively prompts args under "meta.args"
fn prompt_args(template: &Template) -> ProplateResult<Context> {
  let mut ctx = Context::new();
  let TemplateConf { args, .. } = &template.conf;

  for arg in args {
    let input = Input::from(arg);
    ctx.insert(input.get_attr().name.clone(), input.prompt());
  }

  Ok(ctx)
}

/// Executes hook and bind ctx onto dynamic_files.
fn process_template(template: &mut Template, ctx: &Context) -> ProplateResult<()> {
  let TemplateConf {
    additional_operations,
    dynamic_files,
    ..
  } = &template.conf;

  // run "additional_operations" in order to process the dynamically
  // added file in the extra operation.
  if let Some(ops) = &additional_operations {
    for op in ops {
      op.execute(&ctx)?;
    }
  }

  if let Some(dynamic_files) = dynamic_files {
    for filepath in dynamic_files {
      bind_ctx_to_file(Path::new(filepath), ctx);
    }
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
  pfs::copy_fdir(
    src,
    dest,
    template
      .conf
      .exclude
      .clone()
      .map(|vec| vec.iter().map(|s| PathBuf::from(s)).collect::<Vec<_>>()),
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
  fs::remove_dir_all(&fork.base_path)
    .map_err(|_| ProplateError::fs("unable to cleanup tmp...", vec![&fork.base_path]))?;
  Ok(())
}
