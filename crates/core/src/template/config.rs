use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::{Path, PathBuf},
};

use proplate_errors::ProplateError;

use crate::fs::walk::walk_dir_skip;

use super::{
  op::{AdditionalOperation, Operation},
  META_CONF,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum JSONArgType {
  Text,
  Select,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONArg {
  pub key: String,
  pub q_type: JSONArgType,
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
  pub args: Vec<JSONArg>,
  /// List of files containing dynamic variables
  /// used by Proplate to prevent having to go through every template file
  #[serde(default = "Vec::new")]
  pub dynamic_files: Vec<String>,
  #[serde(default = "Vec::new")]
  pub additional_operations: Vec<AdditionalOperation>,

  #[serde(skip)]
  pub dyn_files_reanalyze: bool,
}

impl TemplateConf {
  pub fn new(path: &Path) -> TemplateConf {
    let conf = path.join(META_CONF);
    let meta_json = fs::read_to_string(conf).expect("meta.json can't be located or locked");
    let mut config = parse_config(&meta_json);

    normalize(&mut config, path);

    config
  }
}

fn parse_config(meta_json: &str) -> TemplateConf {
  serde_json::from_str(meta_json)
    .map_err(|e| ProplateError::invalid_template_conf(e.to_string().as_str()))
    .unwrap()
}

fn normalize(config: &mut TemplateConf, base_path: &Path) {
  set_exclude_files(config, base_path);
  set_additional_ops_files(config, base_path);

  config.dyn_files_reanalyze = true;
  // When there are no more operations, we can analyze the dyn files here and indicate that
  // it does not require reanalysis before dyn files ctx binding.
  // Only additional operations have the ability to modify the state of the template files.
  if config.additional_operations.is_empty() {
    analyze_dyn_files(config, base_path);
    config.dyn_files_reanalyze = false;
  }
}

fn set_exclude_files(config: &mut TemplateConf, base_path: &Path) {
  let files = &mut config.exclude;

  // Always exclude meta.json and .proplate_aux_utils
  files.extend([".proplate_aux_utils".into(), META_CONF.into()]);
  to_tmp_files(files, base_path);
}

fn set_additional_ops_files(config: &mut TemplateConf, base_path: &Path) {
  for additional_op in &mut config.additional_operations {
    for op in &mut additional_op.operations {
      match op {
        Operation::Copy { file, dest } => {
          *file = to_tmp_file(PathBuf::from(&file), base_path)
            .display()
            .to_string();
          *dest = to_tmp_file(PathBuf::from(&dest), base_path)
            .display()
            .to_string();
        }
        Operation::Remove { files } => {
          to_tmp_files(files, base_path);
        }
      }
    }
  }
}

pub fn analyze_dyn_files(config: &mut TemplateConf, base_path: &Path) {
  if config.dynamic_files.is_empty() {
    populate_dynamic_files(config, base_path);
  } else {
    update_dynamic_files(config, base_path);
  }
}

fn populate_dynamic_files(config: &mut TemplateConf, base_path: &Path) {
  let TemplateConf {
    dynamic_files,
    exclude,
    ..
  } = config;
  let exclude_paths = exclude.iter().map(|s| PathBuf::from(s)).collect::<Vec<_>>();
  *dynamic_files = walk_dir_skip(base_path, exclude_paths)
    .expect("Walk dir")
    .iter()
    .map(|(file, _)| file.display().to_string())
    .collect::<Vec<_>>();
}

fn update_dynamic_files(config: &mut TemplateConf, base_path: &Path) {
  let TemplateConf {
    dynamic_files,
    exclude,
    ..
  } = config;
  to_tmp_files(dynamic_files, base_path);
  *dynamic_files = dynamic_files
    .into_iter()
    .filter_map(|file| {
      if !exclude.contains(&file) {
        Some(file.to_owned())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
}

fn to_tmp_files(files: &mut Vec<String>, base: &Path) {
  for file in files.into_iter() {
    *file = to_tmp_file(PathBuf::from(&file), base)
      .display()
      .to_string();
  }
}

fn to_tmp_file(path: PathBuf, base: &Path) -> PathBuf {
  base.join(path)
}
