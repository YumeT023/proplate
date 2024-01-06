use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use proplate_errors::ProplateError;

use super::{op::AdditionalOperation, META_CONF};

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
  pub additional_operations: Option<Vec<AdditionalOperation>>,
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
  set_dynamic_files(config, base_path);
}

fn set_exclude_files(config: &mut TemplateConf, base_path: &Path) {
  let files = &mut config.exclude;

  // Always exclude meta.json and .proplate_aux_utils
  files.extend([".proplate_aux_utils".into(), META_CONF.into()]);
  to_tmp_file(files, base_path);
}

fn set_dynamic_files(config: &mut TemplateConf, base_path: &Path) {
  let files = &mut config.dynamic_files;
  to_tmp_file(files, base_path);
}

fn to_tmp_file(files: &mut Vec<String>, base_path: &Path) {
  for file in files.into_iter() {
    let path = base_path.join(&file).display().to_string();
    *file = path;
  }
}
