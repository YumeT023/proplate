use std::{ffi::OsString, path::PathBuf, fmt::Display};

pub mod find;
pub mod error;

#[derive(Debug)]
pub struct Template<'a> {
  pub id: &'a str,
  pub base_path: PathBuf,
  pub base_file_list: Vec<OsString>
}

#[derive(Debug)]
pub struct ForkTemplate<'a> {
  pub original_template: Template<'a>,
  pub tmp_dir: PathBuf
}

const META_CONF: &str = "meta.json";

impl Display for Template<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("TEMPLATE [{}], base_path: {:?}", self.id, self.base_path))
    }
}

impl<'a> Template<'a> {
  pub fn build(id: &'a str, base_path: PathBuf, base_file_list: Vec<OsString>) -> Template<'a> {
    let new = Self {
      id,
      base_path,
      base_file_list
    };
    Template::validate(&new);
    new
  }

  pub fn validate(template: &Template) {
    let filelist = template.base_file_list.clone();
    let mut violations = Vec::<String>::new();

    if !filelist.contains(&OsString::from(META_CONF)) {
      violations.push(String::from("No `meta_json` conf file"));
    }

    if !violations.is_empty() {
      panic!("Error\n{}", violations.join("\n"))
    }
  }
}

impl<'a> ForkTemplate<'a> {
  pub fn new(original_template: Template<'a>, tmp_dir: PathBuf) -> ForkTemplate<'a> {
    Self {
      original_template,
      tmp_dir
    }
  }
}