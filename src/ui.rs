use owo_colors::OwoColorize;

pub fn title(s: &str) -> String {
    format!("{}", s.bold().bright_yellow())
}

pub fn step(s: &str) -> String {
    format!("> {}", s.white())
}

pub fn error(s: &str) -> String {
    format!("{}", s.red())
}

pub fn success(s: &str) -> String {
    format!("{}", s.green())
}

pub trait AsError {
    fn print_err(&self) -> String;
}
