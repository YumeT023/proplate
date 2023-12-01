use std::{fs, path::PathBuf, process};

use serde::{Deserialize, Serialize};

use crate::colors::error;

use super::META_CONF;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateConf {
    template_id: String,
}

pub fn get_template_conf(base_path: PathBuf) -> TemplateConf {
    let path = base_path.join(META_CONF);
    let meta_json = fs::read_to_string(path).expect(&error("Unable to read meta.json"));
    let conf = match serde_json::from_str(&meta_json) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "{}",
                error(&format!("ReadTemplateConfError:\n{}", e.to_string()))
            );
            process::exit(1)
        }
    };
    conf
}
