use owo_colors::OwoColorize;
use proplate_tui::logger::{self, AsError};

#[derive(Debug, Clone)]
pub enum TemplateErrorKind {
  NotFound { is_remote: bool },
  Invalid,
  NoConfig,
}

#[derive(Debug, Clone)]
pub enum CliErrorKind {
  Prompt,
}

#[derive(Debug, Clone)]
pub enum ProplateErrorKind {
  Cli(CliErrorKind),
  Template {
    kind: TemplateErrorKind,
    location: String,
  },
  Fs {
    concerned_paths: Vec<String>,
    operation: String,
  },
  Git {
    cmd: String,
    raw_stderr: String,
  },
}

impl ToString for ProplateErrorKind {
  fn to_string(&self) -> String {
    let str = match self {
      ProplateErrorKind::Cli(_) => "Cli",
      ProplateErrorKind::Template { .. } => "Template",
      ProplateErrorKind::Fs { .. } => "Fs",
      ProplateErrorKind::Git { .. } => "Git",
    };
    str.into()
  }
}

#[derive(Debug)]
pub struct ProplateError {
  kind: ProplateErrorKind,
  cause: Option<String>,
  ctx: Option<String>,
}

pub type ProplateResult<T> = Result<T, ProplateError>;

impl ProplateError {
  pub fn create(kind: ProplateErrorKind) -> ProplateError {
    Self {
      kind,
      cause: None,
      ctx: None,
    }
  }

  pub fn with_ctx(mut self, ctx: &str) -> Self {
    self.ctx = Some(ctx.into());
    self
  }

  pub fn with_cause(mut self, cause: &str) -> Self {
    self.cause = Some(cause.into());
    self
  }

  pub fn has_ctx(&self) -> bool {
    self.ctx.is_some()
  }

  pub fn has_cause(&self) -> bool {
    self.cause.is_some()
  }
}

impl AsError for ProplateError {
  fn print_err(&self) -> String {
    let contextual = match self.kind.clone() {
      ProplateErrorKind::Template { kind, location } => match kind {
        TemplateErrorKind::NotFound { is_remote } => {
          let location_spec = match is_remote {
            true => "remote",
            false => "",
          };
          format!("{} template '{}' cannot be found", location_spec, location)
            .trim()
            .into()
        }
        TemplateErrorKind::Invalid => {
          format!("template at '{}' config (meta.json) is not valid", location)
        }

        TemplateErrorKind::NoConfig => {
          format!("template at '{}' has no config file", location)
        }
      },

      ProplateErrorKind::Cli(kind) => match kind {
        CliErrorKind::Prompt => format!("a problem occured when prompting the user"),
      },

      ProplateErrorKind::Fs {
        concerned_paths,
        operation,
      } => format!(
        "op '{}' cannot be done\n\nConcerned paths are:\n\n{}",
        operation,
        concerned_paths
          .iter()
          .map(|p| format!("- {}", p))
          .collect::<Vec<_>>()
          .join("\n")
      ),

      ProplateErrorKind::Git { cmd, raw_stderr } => {
        format!("command '{}' failed with git err:\n\n{}", cmd, raw_stderr)
      }
    };

    let kind = format!("Error: `{}`", self.kind.to_string());
    let ctx = match self.ctx.clone() {
      Some(_ctx) => format!("\n\nCtx: {}", &_ctx.bold()),
      _ => "".into(),
    };
    let cause = match self.cause.clone() {
      Some(_cause) => format!("\n\nCause:\n{}", &_cause).red().to_string(),
      _ => "".into(),
    };

    logger::error(&format!("\n{kind}\n{contextual}{ctx}{cause}"))
  }
}
