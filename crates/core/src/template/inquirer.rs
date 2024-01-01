use inquire::{error::InquireResult, Select, Text};

use super::config::{JSONArg, JSONArgType};

use proplate_errors::ProplateError;
use proplate_tui::logger::AsError;

pub enum Input<'a> {
  Text(Text<'a>, &'a JSONArg),
  Select(Select<'a, String>, &'a JSONArg),
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
      Input::Text(p, _) => Self::handle_prompt(p.clone().prompt()),
      Input::Select(p, _) => Self::handle_prompt(p.clone().prompt()),
    }
  }

  pub fn arg(&self) -> &JSONArg {
    match self {
      Input::Text(_, arg) => arg,
      Input::Select(_, arg) => arg,
    }
  }
}

impl<'a> From<&'a JSONArg> for Input<'a> {
  fn from(value: &'a JSONArg) -> Self {
    match value.q_type {
      JSONArgType::Text => Input::Text(Text::new(&value.label), value),
      JSONArgType::Select => {
        let options = value.options.clone().unwrap_or_default();
        let select = Select::new(&value.label, options);
        Input::Select(select, value)
      }
    }
  }
}
