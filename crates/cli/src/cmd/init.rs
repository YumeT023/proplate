use proplate_core::{
  gen::bootstrap::{bootstrap, Context},
  local::get_local_template,
  template::resolver::clone_template,
};
use proplate_errors::ProplateResult;
use proplate_tui::logger;

pub fn init(id: String, dest: Option<String>) -> ProplateResult<String> {
  println!("{}", logger::title("Initializing template"));

  let path_to_clone = get_local_template("tiniest".to_string());

  let dest = dest.unwrap_or(id.clone());

  let mut template = clone_template(path_to_clone.display().to_string().as_str(), &dest)?;

  let ctx = Context::from([("id".to_string(), id)]);

  bootstrap(&mut template, &dest, &ctx)?;

  Ok("".to_string())
}
