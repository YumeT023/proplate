use std::{ffi::OsString, fmt::Display, path::PathBuf};

use self::conf::{get_template_conf, TemplateConf};
use crate::ui;

pub mod conf;
pub mod find;

#[derive(Debug)]
pub struct Template {
    pub id: String,
    pub base_path: PathBuf,
    pub base_file_list: Vec<OsString>,
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
        base_file_list: Vec<OsString>,
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

    fn validate_filelist(filelist: &Vec<OsString>) {
        let mut violations = Vec::<String>::new();

        if !filelist.contains(&OsString::from(META_CONF)) {
            violations.push(String::from("No `meta_json` conf file"));
        }

        if !violations.is_empty() {
            panic!("Error\n{}", ui::error(&violations.join("\n")))
        }
    }
}
