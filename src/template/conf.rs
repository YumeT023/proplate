use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    errors::ProplateError,
    settings::JSONArg,
    ui::{self, AsError},
};

use super::META_CONF;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConf {
    pub id: String,
    pub args: Vec<JSONArg>,
    pub dynamic_files: Option<Vec<String>>,
}

pub fn get_template_conf(base_path: PathBuf) -> TemplateConf {
    let path = base_path.join(META_CONF);
    let meta_json = fs::read_to_string(path).expect(&ui::error("Unable to read meta.json"));

    match serde_json::from_str(&meta_json) {
        Ok(conf) => conf,
        Err(e) => panic!(
            "{}",
            ProplateError::invalid_template_conf(&e.to_string()).print_err()
        ),
    }
}
