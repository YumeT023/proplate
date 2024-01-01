use proplate_tui::logger;

#[derive(Debug)]
pub enum ProplateErrorKind {
  TemplateNotFound,
  InvalidTemplate,
  Fs,
  Git,
  PromptUser,
}

#[derive(Debug)]
pub struct ProplateError {
  pub kind: ProplateErrorKind,
  pub reason: String,
}

pub type ProplateResult<T> = Result<T, ProplateError>;

impl ProplateError {
  pub fn new(kind: ProplateErrorKind, reason: &str) -> ProplateError {
    Self {
      kind,
      reason: reason.to_string(),
    }
  }

  pub fn invalid_template_conf(details: &str) -> ProplateError {
    Self::new(ProplateErrorKind::InvalidTemplate, details)
  }

  pub fn fs(details: &str) -> ProplateError {
    Self::new(ProplateErrorKind::Fs, details)
  }

  pub fn local_template_not_found(path: &str) -> ProplateError {
    Self::new(
      ProplateErrorKind::TemplateNotFound,
      &format!("Local template (dir={}) is not found.", path),
    )
  }

  pub fn remote_template_not_found(url: &str) -> ProplateError {
    Self::new(
      ProplateErrorKind::TemplateNotFound,
      &format!("Remote template (url={}) is not found.", url),
    )
  }

  pub fn prompt(details: &str) -> ProplateError {
    Self::new(ProplateErrorKind::PromptUser, details)
  }

  pub fn git(details: &str) -> ProplateError {
    Self::new(ProplateErrorKind::Git, details)
  }
}

impl logger::AsError for ProplateError {
  fn print_err(&self) -> String {
    logger::error(&format!("{:?}", self))
  }
}
