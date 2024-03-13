use proplate_core::{
  gen::bootstrap::{bootstrap, Context},
  local::get_local_template,
  template::resolver::clone_template,
};
use proplate_errors::ProplateResult;
use proplate_tui::logger;

const INIT_TEMPLATE_GIT_REPO: &str = "https://github.com/YumeT023/tiniest-proplate";

const CANNOT_INIT_LOCALLY_WARNING_MSG: &str = r#"Unable to find proplate 'builtins' dir so we're going to init your template through git repo instead.
To remove this warning, either download proplate from gha release so it comes with the 'builtins' dir automatically or manually get the 'builtins' dir from proplate repo https://github.com/YumeT023/proplate."#;


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
