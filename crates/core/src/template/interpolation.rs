use regex::Regex;
use std::collections::HashMap;

const INTERPOLATION_PATTERN: &str = r"(\\*)\$([a-zA-Z_][a-zA-Z0-9_]*)\b";

fn create_regex() -> Regex {
  Regex::new(INTERPOLATION_PATTERN).unwrap()
}

/// Regex-based var binding replacement inside of a string slice
///
/// # Example
/// ```
/// let mut ctx = HashMap::new();
/// ctx.insert("name".to_string(), "proplate".to_string());
/// println!("{}", provide_ctx("Hello $name", Some(ctx))) // "Hello proplate"
/// ````
fn provide_ctx(source: &str, ctx: Option<HashMap<String, String>>) -> String {
  let ctx = ctx.unwrap_or_default();
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

pub trait MapWithCtx {
  fn map_with_ctx(&self, ctx: Option<HashMap<String, String>>) -> Self;
}

impl MapWithCtx for String {
  fn map_with_ctx(&self, ctx: Option<HashMap<String, String>>) -> Self {
    provide_ctx(self, ctx)
  }
}
