use inquire::Confirm;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use proplate_core::{
    fs as pfs,
    template::{
        conf::inquire_adapter::AskUser,
        interpolation::provide_ctx,
        resolver::find_template,
        types::{Template, META_CONF},
    },
};
use proplate_errors::{ProplateError, ProplateResult};
use proplate_integration::git;
use proplate_tui::logger::{self, AsError};

#[derive(Debug, Default)]
pub struct CreateOptions {
    pub git: bool,
}

pub fn create(from: &str, dest: &str, options: CreateOptions) -> ProplateResult<()> {
    println!("{}", logger::title("Setup template"));
    let fork = fork_template(from, dest)?;

    let cleanup = || {
        println!("{}", logger::step("cleaning up..."));
        fs::remove_dir_all(&fork.base_path)
            .expect(&ProplateError::fs("unable to cleanup tmp...").print_err())
    };

    initialize_template(&fork)?;

    options.git.then(|| init_git_repo(&fork.base_path));

    fs::create_dir_all(dest).map_err(|e| {
        cleanup();
        ProplateError::fs(&format!("{}", e.to_string()))
    })?;

    println!("{}", logger::title("Finalizing"));
    println!("{}", logger::step("Copying..."));
    pfs::copy_directory(&fork.base_path, Path::new(dest)).map_err(|e| {
        cleanup();
        ProplateError::fs(&format!("{}", e.to_string()))
    })?;

    cleanup();

    Ok(())
}

fn fork_template(from: &str, dest: &str) -> ProplateResult<Template> {
    println!("{}", logger::step("Finding template..."));
    let mut template = match find_template(from) {
        Ok(t) => t,
        Err(e) => panic!("{}", e.print_err()),
    };

    if template.fork_source.is_none() {
        let path_str = format!(".temp/{}-{}", dest, Uuid::new_v4());
        let path_buf = PathBuf::from(path_str);

        fs::create_dir_all(&path_buf)
            .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

        println!("{}", logger::step("Forking template..."));
        pfs::copy_directory(&template.base_path, path_buf.as_path())
            .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

        template.base_path = path_buf;
    } else {
        println!(
            "{}",
            logger::step(&format!("Cloned template repo: {}", from))
        )
    }

    Ok(template)
}

fn initialize_template(template: &Template) -> ProplateResult<()> {
    let mut ctx: HashMap<String, String> = HashMap::new();

    println!("{}", logger::title("Template initialization:"));
    template
        .conf
        .args
        .iter()
        .map(|arg| AskUser::from(arg))
        .for_each(|q| {
            ctx.insert(q.arg().key.to_string(), q.prompt());
        });

    let dynamic_files = template.conf.dynamic_files.clone().unwrap_or_default();

    println!("{}", logger::step("replacing vars in dynamic files..."));
    for file_path in dynamic_files {
        println!(
            "      {}",
            logger::step(&format!("processing {}", &file_path))
        );
        let relative_path = template.base_path.join(file_path);
        pfs::map_file(Path::new(&relative_path), |c| {
            provide_ctx(c, Some(ctx.clone()))
        })
        .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;
    }

    println!("{}", logger::step("Deleting unused files..."));
    fs::remove_file(template.base_path.join(META_CONF))
        .map_err(|e| ProplateError::fs(&format!("{}", e.to_string())))?;

    Ok(())
}

fn init_git_repo(path: &Path) -> ProplateResult<()> {
    let lockfile = path.join(".git");

    if lockfile.exists() {
        let init_anyway = Confirm::new(
            "Git is already initialized in the template, Do you want to reinitialize ?",
        )
        .prompt()
        .map_err(|e| ProplateError::prompt(&e.to_string()))?;
        if !init_anyway {
            return Ok(());
        }
        fs::remove_dir_all(lockfile).map_err(|e| ProplateError::fs(&e.to_string()))?
    }

    println!("{}", logger::title("Initializing git repo"));
    git::exec_cmd(["init"], path)?;
    git::exec_cmd(["add", "-A"], path)?;
    git::exec_cmd(
        ["commit", "-m", "chore: initial commit", "--allow-empty"],
        path,
    )?;
    Ok(())
}