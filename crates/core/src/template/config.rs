use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use proplate_errors::ProplateError;
use proplate_tui::logger::{self, AsError};

use super::{conditional::ConditionalOperation, META_CONF};

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
  /// Arguments that Proplate will ask when a project is created using the associated template
  pub args: Vec<JSONArg>,
  /// List of files containing dynamic variables
  /// used by Proplate to prevent having to go through every template file
  pub dynamic_files: Option<Vec<String>>,
  pub conditional_operations: Option<Vec<ConditionalOperation>>,
}

pub fn get_template_conf(base_path: PathBuf) -> TemplateConf {
  let path = base_path.join(META_CONF);
  let meta_json = fs::read_to_string(path).expect(&logger::error("Unable to read meta.json"));

  match serde_json::from_str(&meta_json) {
    Ok(conf) => conf,
    Err(e) => panic!(
      "{}",
      ProplateError::invalid_template_conf(&e.to_string()).print_err()
    ),
  }
}
