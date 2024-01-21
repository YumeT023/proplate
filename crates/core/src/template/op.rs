use std::{collections::HashMap, fs, path::Path};

use proplate_errors::{ProplateError, ProplateResult};
use serde::{Deserialize, Serialize};

use super::interpolation::MapWithCtx;
use crate::fs as pfs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StringCompareOp {
  Eq,
  NotEqual,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
  pub lhs: String,
  pub op: StringCompareOp,
  pub rhs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
  // Separate op to avoid ambiguity
  Copy { file: String, dest: String },
  CopyDir { path: String, dest: String },
  Remove { files: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalOperation {
  #[serde(default = "Vec::new")]
  pub conditions: Vec<Condition>,
  /// operations to execute if the above conditions are evaluated as true
  pub operations: Vec<Operation>,
}

impl Condition {
  fn eval(&self) -> bool {
    match self.op {
      StringCompareOp::Eq => self.lhs == self.rhs,
      StringCompareOp::NotEqual => self.lhs != self.rhs,
    }
  }

  pub fn eval_in_ctx(&self, ctx: &HashMap<String, String>) -> bool {
    let mut c = self.clone();
    c.lhs = c.lhs.map_with_ctx(ctx);
    c.rhs = c.rhs.map_with_ctx(ctx);
    c.eval()
  }
}

pub trait Execute {
  fn execute(&self, ctx: &HashMap<String, String>) -> ProplateResult<()>;
}

impl Execute for Operation {
  fn execute(&self, _ctx: &HashMap<String, String>) -> ProplateResult<()> {
    match self {
      Operation::Copy { file, dest } => {
        let src = Path::new(&file);
        let dest = Path::new(&dest);
        fs::copy(src, dest).map_err(|e| ProplateError::fs(&e.to_string(), vec![&src, &dest]))?;
        Ok(())
      }
      Operation::CopyDir { path, dest } => {
        let path = Path::new(path);
        let dest = Path::new(dest);
        pfs::copy_fdir(path, dest, None).expect("copy_dir");
        Ok(())
      }
      Operation::Remove { files } => {
        for file in files {
          let src = Path::new(&file);
          pfs::remove_fdir(src).map_err(|e| ProplateError::fs(&e.to_string(), vec![&src]))?;
        }
        Ok(())
      }
    }
  }
}

impl Execute for AdditionalOperation {
  fn execute(&self, ctx: &HashMap<String, String>) -> ProplateResult<()> {
    // eval condition or true if it is empty or missing
    let conditions = &self.conditions;
    let true_ = match conditions.is_empty() {
      true => true,
      false => conditions.iter().all(|c| c.eval_in_ctx(ctx)),
    };

    if true_ {
      for operation in &self.operations {
        operation.execute(ctx)?;
      }
    }

    Ok(())
  }
}
