use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::types::META_CONF;
use proplate_errors::ProplateError;

use proplate_tui::logger::{self, AsError};

pub mod inquire_adapter;

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
    pub is_required: Option<bool>,
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConf {
    pub id: String,
    pub args: Vec<JSONArg>,
    pub dynamic_files: Option<Vec<String>>,
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
