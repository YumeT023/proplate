use std::{fmt::Display, path::PathBuf};

use proplate_tui::logger;

use self::config::{get_template_conf, TemplateConf};

pub mod config;
pub mod inquirer;
pub mod interpolation;
pub mod resolver;

#[derive(Debug)]
pub struct Template {
  pub id: String,
  /// Template path, which may be either the forked or local template path
  pub base_path: PathBuf,
  pub base_file_list: Vec<String>,
  /// Github repo if he template is from github
  pub fork_source: Option<String>,
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
    fork_source: Option<String>,
  ) -> Template {
    Template::validate_filelist(&base_file_list);
    Self {
      id,
      base_path: base_path.clone(),
      base_file_list,
      fork_source,
      conf: get_template_conf(base_path),
    }
  }

  fn validate_filelist(filelist: &Vec<String>) {
    let mut violations = Vec::<String>::new();

    if !filelist.contains(&META_CONF.to_string()) {
      violations.push(String::from("No `meta_json` conf file"));
    }

    if !violations.is_empty() {
      panic!("Error\n{}", logger::error(&violations.join("\n")))
    }
  }
}
