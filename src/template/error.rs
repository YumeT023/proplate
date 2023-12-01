use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    TemplateNotFoundError(String),
}

impl Error {
    pub fn not_found(id: String) -> Error {
        Error::TemplateNotFoundError(id)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TemplateNotFoundError(id) => {
                f.write_fmt(format_args!("template [{}] doesn't exist", id))
            }
        }
    }
}
