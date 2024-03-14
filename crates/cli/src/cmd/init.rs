use proplate_core::{
  gen::bootstrap::{bootstrap, Context},
  local::get_local_template,
  template::resolver::clone_template,
};
use proplate_errors::ProplateResult;
use proplate_tui::logger;

const INIT_TEMPLATE_GIT_REPO: &str = "https://github.com/YumeT023/tiniest-proplate";

const CANNOT_INIT_LOCALLY_WARNING_MSG: &str = r#"Unable to find proplate 'builtins' dir so we're going to init your template through git repo instead.
To remove this warning, download builtins.zip directory from https://github.com/YumeT023/proplate/releases and extract it in the same dir as proplate executable"#;

pub fn init(id: String, dest: Option<String>) -> ProplateResult<String> {
  let dest = dest.unwrap_or(id.clone());

  let to_clone = match get_local_template("tiniest") {
    Ok(path) => path.display().to_string(),
    _ => {
      println!("\n{}", logger::warn(CANNOT_INIT_LOCALLY_WARNING_MSG));
      INIT_TEMPLATE_GIT_REPO.to_string()
    }
  };

  println!("{}", logger::title("Initializing template"));

  let mut template = clone_template(&to_clone, &dest)?;
  let ctx = Context::from([("id".to_string(), id)]);

  bootstrap(&mut template, &dest, &ctx)?;

  Ok("".to_string())
}
