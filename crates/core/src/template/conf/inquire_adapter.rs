use inquire::{error::InquireResult, Select, Text};

use proplate_errors::ProplateError;
use proplate_tui::logger::AsError;

use super::{JSONArg, JSONArgType};

pub enum AskUser<'a> {
    Text(Text<'a>, &'a JSONArg),
    Select(Select<'a, String>, &'a JSONArg),
}

impl<'a> AskUser<'a> {
    fn handle_prompt(result: InquireResult<String>) -> String {
        match result {
            Ok(t) => t,
            Err(e) => panic!("{}", ProplateError::prompt(&e.to_string()).print_err()),
        }
    }

    pub fn prompt(&self) -> String {
        match self {
            AskUser::Text(text, _) => AskUser::handle_prompt(text.clone().prompt()),
            AskUser::Select(select, _) => AskUser::handle_prompt(select.clone().prompt()),
        }
    }

    pub fn arg(&self) -> &JSONArg {
        match self {
            AskUser::Text(_, arg) => arg,
            AskUser::Select(_, arg) => arg,
        }
    }
}

impl<'a> From<&'a JSONArg> for AskUser<'a> {
    fn from(value: &'a JSONArg) -> Self {
        match value.q_type {
            JSONArgType::Text => AskUser::Text(Text::new(&value.label), value),
            JSONArgType::Select => {
                let options = value.options.clone().unwrap_or_default();
                let select = Select::new(&value.label, options);
                AskUser::Select(select, value)
            }
        }
    }
}
