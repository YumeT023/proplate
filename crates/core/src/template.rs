use std::{fmt::Display, path::PathBuf, process::exit};

use proplate_errors::{ProplateError, ProplateErrorKind, TemplateErrorKind};
use proplate_tui::logger::AsError;

use self::config::TemplateConf;

pub mod config;
pub mod inquirer;
pub mod interpolation;
pub mod op;
pub mod resolver;

#[derive(Debug)]
pub struct Template {
  pub id: String,
  /// Template path, which may be either the forked or local template path
  pub base_path: PathBuf,
  pub base_file_list: Vec<String>,
  /// Github repo if he template is from github
  pub fork_source: String,
  pub conf: TemplateConf,
}

pub const META_CONF: &str = "meta.json";

impl Display for Template {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "TEMPLATE [{}], base_path: {:?}",
      self.id, self.base_path
    ))
  }
}

impl Template {
  pub fn build(
    id: String,
    base_path: PathBuf,
    base_file_list: Vec<String>,
    fork_source: String,
  ) -> Template {
    Template::validate_template_filebase(&base_file_list, fork_source.clone());
    Template {
      id,
      base_path: base_path.clone(),
      base_file_list,
      fork_source,
      conf: TemplateConf::new(&base_path),
    }
  }

  /// Validates main files
  /// Namely ensures that meta.json is present
  fn validate_template_filebase(files: &Vec<String>, location: String) {
    let mut violations = Vec::<String>::new();

    if !files.contains(&META_CONF.to_string()) {
      violations.push(String::from("No `meta_json` conf file"));
    }

    if !violations.is_empty() {
      eprintln!(
        "{}",
        ProplateError::create(ProplateErrorKind::Template {
          kind: TemplateErrorKind::NoConfig,
          location
        })
        .with_ctx("template:validate")
        .print_err()
      );
      exit(1);
    }
  }
}
