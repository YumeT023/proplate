use inquire::{Select, Text};

use super::{JSONArg, JSONArgType};

pub enum AskUser<'a> {
    Text(Text<'a>, &'a JSONArg),
    Select(Select<'a, String>, &'a JSONArg),
}

impl<'a> AskUser<'a> {
    pub fn prompt(&self) -> String {
        match self {
            AskUser::Text(text, arg) => text.clone().prompt().expect(&format!("arg {}", arg.key)),
            AskUser::Select(select, arg) => {
                select.clone().prompt().expect(&format!("arg {}", arg.key))
            }
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
