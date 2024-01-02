use inquire::{error::InquireResult, Select, Text};

use super::config::{JSONArg, JSONArgType};

use proplate_errors::ProplateError;
use proplate_tui::logger::AsError;

/// For mapping input attribute internally
pub struct InputAttr {
  /// default value
  pub default: Option<String>,
  pub name: String,
}

pub enum Input<'a> {
  Text(Text<'a>, InputAttr),
  Select(Select<'a, String>, InputAttr),
}

impl<'a> Input<'a> {
  fn handle_prompt(result: InquireResult<String>) -> String {
    match result {
      Ok(t) => t,
      Err(e) => panic!("{}", ProplateError::prompt(&e.to_string()).print_err()),
    }
  }

  pub fn prompt(&self) -> String {
    match self {
      Input::Text(p, attr) => {
        let p = p.clone();
        Self::handle_prompt(
          p.with_initial_value(attr.default.clone().unwrap_or("".to_string()).as_ref())
            .prompt(),
        )
      }
      Input::Select(p, _) => Self::handle_prompt(p.clone().prompt()),
    }
  }

  pub fn get_attr(&self) -> &InputAttr {
    match self {
      Input::Select(_, attr) | Input::Text(_, attr) => attr,
    }
  }
}

impl<'a> From<&'a JSONArg> for Input<'a> {
  fn from(value: &'a JSONArg) -> Self {
    match value.q_type {
      JSONArgType::Text => {
        let attr = InputAttr {
          default: value.default_value.clone(),
          name: value.key.clone(),
        };
        Input::Text(Text::new(&value.label), attr)
      }
      JSONArgType::Select => {
        let options = value.options.clone().unwrap_or_default();
        let attr = InputAttr {
          default: None,
          name: value.key.clone(),
        };
        Input::Select(Select::new(&value.label, options), attr)
      }
    }
  }
}
