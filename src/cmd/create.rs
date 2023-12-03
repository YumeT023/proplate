use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::{
    errors::{ProplateError, ProplateResult},
    settings::adapter::AskUser,
    shell,
    template::{find::find_template_by_id, Template, META_CONF},
    ui::{self, AsError},
    util::interpolate::provide_ctx,
    wrapper::exec_git_cmd,
};

#[derive(Debug, Default)]
pub struct CreateOptions {
    pub git: bool,
}

pub fn create(template_id: &str, dest: &str, options: CreateOptions) -> ProplateResult<()> {
    println!("{}", ui::title("Setup template"));
    let fork = fork_template(template_id, dest)?;

    let cleanup = || {
        println!("{}", ui::step("cleaning up..."));
        fs::remove_dir_all(&fork.base_path)
            .expect(&ProplateError::fs("unable to cleanup tmp...").print_err())
    };

    initialize_template(&fork)?;

    options.git.then(|| init_git_repo(&fork.base_path));

    fs::create_dir_all(dest).map_err(|e| {
        cleanup();
        ProplateError::fs(&format!("{}", e.to_string()))
    })?;

    println!("{}", ui::title("Finalizing"));
    println!("{}", ui::step("Copying..."));
    shell::copy_directory(&fork.base_path, Path::new(dest)).map_err(|e| {
        cleanup();
        ProplateError::fs(&format!("{}", e.to_string()))
    })?;

    cleanup();

    Ok(())
}

fn fork_template(id: &str, dest: &str) -> ProplateResult<Template> {
    println!("{}", ui::step("Finding template..."));
    let mut template = match find_template_by_id(id) {
        Ok(t) => t,
        Err(e) => panic!("{}", e.print_err()),
    };

    let path_str = format!(".temp/{}-{}", dest, Uuid::new_v4());
    let path_buf = PathBuf::from(path_str);

    fs::create_dir_all(&path_buf).map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

    println!("{}", ui::step("Forking template..."));
    shell::copy_directory(&template.base_path, path_buf.as_path())
        .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

    template.base_path = path_buf;

    Ok(template)
}

fn initialize_template(template: &Template) -> ProplateResult<()> {
    let mut ctx: HashMap<String, String> = HashMap::new();

    println!("{}", ui::title("Template initialization:"));
    template
        .conf
        .args
        .iter()
        .map(|arg| AskUser::from(arg))
        .for_each(|q| {
            ctx.insert(q.arg().key.to_string(), q.prompt());
        });

    let dynamic_files = template.conf.dynamic_files.clone().unwrap_or_default();

    println!("{}", ui::step("replacing vars in dynamic files..."));
    for file_path in dynamic_files {
        println!("      {}", ui::step(&format!("processing {}", &file_path)));
        let relative_path = template.base_path.join(file_path);
        shell::map_file(Path::new(&relative_path), |c| {
            provide_ctx(c, Some(ctx.clone()))
        })
        .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;
    }

    println!("{}", ui::step("Deleting unused files..."));
    fs::remove_file(template.base_path.join(META_CONF))
        .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

    Ok(())
}

fn init_git_repo(path: &Path) -> ProplateResult<()> {
    println!("{}", ui::title("Initializing git repo"));
    exec_git_cmd(["init"], path)?;
    exec_git_cmd(["add", "-A"], path)?;
    exec_git_cmd(
        ["commit", "-m", "chore: initial commit", "--allow-empty"],
        path,
    )?;
    Ok(())
}
