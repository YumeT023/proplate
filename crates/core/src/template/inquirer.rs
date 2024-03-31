use std::process::exit;

use inquire::{error::InquireResult, Select, Text};
use proplate_tui::logger::AsError;

use super::config::{Arg, ArgType};

use proplate_errors::{CliErrorKind, ProplateError, ProplateErrorKind};

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
      Err(e) => {
        // is it really necessary to handle the error here ? if no, return Result<String> then
        // handle it there
        eprintln!(
          "{}",
          ProplateError::create(ProplateErrorKind::Cli(CliErrorKind::Prompt))
            .with_cause(&e.to_string())
            .print_err()
        );
        exit(1);
      }
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

impl<'a> From<&'a Arg> for Input<'a> {
  fn from(value: &'a Arg) -> Self {
    match value.q_type {
      ArgType::Text => {
        let attr = InputAttr {
          default: value.default_value.clone(),
          name: value.key.clone(),
        };
        Input::Text(Text::new(&value.label), attr)
      }
      ArgType::Select => {
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
