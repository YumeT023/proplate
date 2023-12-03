use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::errors::{ProplateError, ProplateResult};

use super::Template;

const BUILT_IN_TEMPLATE_DIR: &str = "built_in";

pub fn find_template_by_id(id: &str) -> ProplateResult<Template> {
    let path = get_template_path_by_id(id);

    if !path.exists() {
        ProplateError::local_template_not_found(id);
    }

    match read_dir(&path) {
        Ok(entries) => {
            let file_list = entries
                .into_iter()
                .filter_map(|e| match e {
                    Ok(entry) => Some(entry.file_name()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            Ok(Template::build(
                id.to_string(),
                path.clone(),
                file_list,
                path.to_str().map(|s| s.to_string()),
            ))
        }
        _ => Err(ProplateError::local_template_not_found(id)),
    }
}

fn get_template_path_by_id(id: &str) -> PathBuf {
    Path::new(BUILT_IN_TEMPLATE_DIR).join(Path::new(id))
}
