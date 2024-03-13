use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::{Path, PathBuf},
};

use proplate_errors::ProplateError;

use crate::fs::walk::{walk_dir, walk_dir_skip};

use super::{
  op::{AdditionalOperation, Operation},
  META_CONF,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgType {
  Text,
  Select,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arg {
  pub key: String,
  pub q_type: ArgType,
  pub label: String,
  pub default_value: Option<String>,
  /// Only used when "key" equals "Select."
  pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConf {
  /// Template id
  pub id: String,
  /// Auxiliary proplate utils
  /// for example, a "License" file that is only copied if the "License" arg is set to "MIT"
  #[serde(default = "Vec::new")]
  pub exclude: Vec<String>,
  /// Arguments that Proplate will ask when a project is created using the associated template
  pub args: Vec<Arg>,
  /// List of files containing dynamic variables
  /// used by Proplate to prevent having to go through every template file
  #[serde(default = "Vec::new")]
  pub dynamic_files: Vec<String>,
  #[serde(default = "Vec::new")]
  pub additional_operations: Vec<AdditionalOperation>,

  #[serde(default = "TemplateConf::default_keep_meta")]
  pub keep_meta: bool,

  /// Prevent examining dyn files repeatedly.
  #[serde(skip)]
  pub require_dyn_file_analysis: bool,
}

impl TemplateConf {
  pub fn new(path: &Path) -> TemplateConf {
    let conf = path.join(META_CONF);
    let meta_json = fs::read_to_string(conf).expect("meta.json can't be located or locked");
    let mut config = parse_config(&meta_json);

    normalize(&mut config, path);

    config
  }

  fn default_keep_meta() -> bool {
    false
  }
}

fn parse_config(meta_json: &str) -> TemplateConf {
  serde_json::from_str(meta_json)
    .map_err(|e| ProplateError::invalid_template_conf(e.to_string().as_str()))
    .unwrap()
}

fn normalize(config: &mut TemplateConf, base: &Path) {
  set_exclude_files(config, base);
  set_additional_ops_files(config, base);

  config.require_dyn_file_analysis = true;
  // Avoid unnecessary analysis
  // As only additional_operationns has the power to change the state of the template files, we can
  // analyze the dyn files here in the absence of any operations and say that no analysis is necessary prior to the dyn files' ctx binding.
  if config.additional_operations.is_empty() {
    analyze_dyn_files(config, base);
    config.require_dyn_file_analysis = false;
  }
}

fn set_exclude_files(config: &mut TemplateConf, base: &Path) {
  let files = &mut config.exclude;

  // Always exclude '.proplate_aux_utils' folder
  files.extend([".proplate_aux_utils".into(), ".git".into()]);

  if !config.keep_meta {
    files.push(META_CONF.into());
  }

  to_relative_all(files, base);
}

fn set_additional_ops_files(config: &mut TemplateConf, base: &Path) {
  for additional_op in &mut config.additional_operations {
    for op in &mut additional_op.operations {
      match op {
        Operation::Copy { file, dest } => {
          *file = to_relative(PathBuf::from(&file), base);
          *dest = to_relative(PathBuf::from(&dest), base);
        }
        Operation::CopyDir { path, dest } => {
          *path = to_relative(PathBuf::from(&path), base);
          *dest = to_relative(PathBuf::from(&dest), base);
        }
        Operation::Remove { files } => {
          to_relative_all(files, base);
        }
      }
    }
  }
}

pub fn analyze_dyn_files(config: &mut TemplateConf, base: &Path) {
  if config.dynamic_files.is_empty() {
    populate_dynamic_files(config, base);
  } else {
    update_dynamic_files(config, base);
  }
}

/// Walks the template files to populate "dynamic_files".
fn populate_dynamic_files(config: &mut TemplateConf, base: &Path) {
  let TemplateConf {
    dynamic_files,
    exclude,
    ..
  } = config;
  let exclude_paths = exclude.iter().map(|s| PathBuf::from(s)).collect::<Vec<_>>();
  *dynamic_files = walk_dir_skip(base, exclude_paths)
    .expect("Walk dir")
    .iter()
    .map(|(file, _)| file.display().to_string())
    .collect::<Vec<_>>();
}

fn update_dynamic_files(config: &mut TemplateConf, base: &Path) {
  let TemplateConf {
    dynamic_files,
    exclude,
    ..
  } = config;
  to_relative_all(dynamic_files, base /* to */);

  let mut expanded = Vec::new();

  // recursively expand the dynamic files
  for path in dynamic_files.iter() {
    if let Ok(files) = walk_dir(Path::new(path)) {
      let paths = files
        .into_iter()
        .filter_map(|(file, _)| {
          let file = file.display().to_string();
          if exclude.contains(&file) {
            return None;
          }
          Some(file)
        })
        .collect::<Vec<_>>();
      expanded.extend(paths);
    }
  }

  dynamic_files.extend(expanded);
}

fn to_relative_all(files: &mut Vec<String>, to: &Path) {
  for file in files.into_iter() {
    *file = to_relative(PathBuf::from(&file), to);
  }
}

fn to_relative(path: PathBuf, to: &Path) -> String {
  to.join(path).display().to_string()
}
