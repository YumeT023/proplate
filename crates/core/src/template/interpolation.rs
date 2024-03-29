use regex::Regex;

use crate::gen::bootstrap::Context;

const INTERPOLATION_PATTERN: &str = r"(\\*)\$([a-zA-Z_][a-zA-Z0-9_]*)\b";

fn create_regex() -> Regex {
  Regex::new(INTERPOLATION_PATTERN).unwrap()
}

/// Regex-based var binding replacement inside of a string slice
///
/// # Example
/// ```
/// use proplate_core::{gen::bootstrap::Context, template::interpolation::interpolate};
///
/// let mut ctx = Context::new();
/// ctx.insert("name".to_string(), "proplate".to_string());
/// println!("{}", interpolate("Hello $name", &ctx)); // "Hello proplate"
/// ````
pub fn interpolate(source: &str, ctx: &Context) -> String {
  let re = create_regex();

  let mut result = String::new();
  let mut last_end = 0;

  for caps in re.captures_iter(source) {
    let escape = caps.get(1).unwrap().as_str();
    let name = caps.get(2).unwrap().as_str();

    let escape_len = escape.len();
    let value = ctx.get(name).map(|v| v.as_str()).unwrap_or("");

    let unescaped_match = &source[last_end..caps.get(0).unwrap().start()];
    result.push_str(unescaped_match);

    if escape_len % 2 != 0 {
      result.push_str(&format!(
        "{}${}",
        escape[..escape_len - 1].to_string(),
        name
      ));
    } else {
      result.push_str(&format!("{}{}", escape, value));
    }

    last_end = caps.get(0).unwrap().end();
  }

  result.push_str(&source[last_end..]);
  result
}

pub trait Interpolate {
  fn interpolate(&self, ctx: &Context) -> Self;
}

/// Allow to use
impl Interpolate for String {
  fn interpolate(&self, ctx: &Context) -> Self {
    interpolate(self, ctx)
  }
}
