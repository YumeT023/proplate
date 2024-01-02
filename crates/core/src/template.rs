use std::{fmt::Display, path::PathBuf};

use proplate_tui::logger;

use crate::fs::canonic_path_from_str_vec;

use self::{
  condition::Operation,
  config::{get_template_conf, TemplateConf},
};

pub mod condition;
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
    Template::validate_template_filebase(&base_file_list);
    let mut template = Template {
      id,
      base_path: base_path.clone(),
      base_file_list,
      fork_source,
      conf: get_template_conf(base_path),
    };
    Self::normalize_template(&mut template);
    template
  }

  /// Prettifies template
  /// write present path to its canonical form
  fn normalize_template(template: &mut Template) {
    Self::_normalize_conditional_operations(template);
    Self::_normalize_dynamic_files(template);
  }

  /// Normalizes the paths in dynamic_files
  fn _normalize_dynamic_files(template: &mut Template) {
    let config = &mut template.conf;
    if let Some(dynamic_files) = &mut config.dynamic_files {
      for file in dynamic_files {
        *file = template
          .base_path
          .join(PathBuf::from(file.as_str()))
          .to_str()
          .map(|s| s.to_string())
          .unwrap();
      }
    }
  }

  /// Normalizes the paths in conditional_operations
  fn _normalize_conditional_operations(template: &mut Template) {
    let config = &mut template.conf;
    let base_path = template.base_path.to_str().unwrap().to_string();
    if let Some(conditional_ops) = &mut config.conditional_operations {
      for ops in conditional_ops {
        for op in &mut ops.operations {
          match op {
            Operation::Copy { files, dest } => {
              *dest = canonic_path_from_str_vec(vec![base_path.clone(), dest.to_string()])
                .unwrap()
                .to_string_lossy()
                .to_string();
              for file in files {
                *file = canonic_path_from_str_vec(vec![base_path.clone(), file.to_string()])
                  .unwrap()
                  .to_string_lossy()
                  .to_string();
              }
            }
          }
        }
      }
    }
  }

  /// Validates main files
  /// Namely ensures that meta.json is present
  fn validate_template_filebase(files: &Vec<String>) {
    let mut violations = Vec::<String>::new();

    if !files.contains(&META_CONF.to_string()) {
      violations.push(String::from("No `meta_json` conf file"));
    }

    if !violations.is_empty() {
      panic!("Error\n{}", logger::error(&violations.join("\n")))
    }
  }
}
