use std::{collections::HashMap, path::Path};

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
  Copy { files: Vec<String>, dest: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConditionalOperation {
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

  pub fn eval_in_ctx(&self, ctx: HashMap<String, String>) -> bool {
    let mut c = self.clone();
    c.lhs = c.lhs.map_with_ctx(Some(ctx.clone()));
    c.rhs = c.rhs.map_with_ctx(Some(ctx));
    c.eval()
  }
}

pub trait Execute {
  fn execute(&self, ctx: HashMap<String, String>) -> ProplateResult<()>;
}

impl Execute for Operation {
  fn execute(&self, _ctx: HashMap<String, String>) -> ProplateResult<()> {
    match self {
      Operation::Copy { files, dest } => {
        for file in files {
          let (file, dest) = (Path::new(&file), Path::new(&dest));
          pfs::copy_fdir(&file, &Path::new(dest))
            .map_err(|e| ProplateError::fs(&e.to_string(), vec![&file, &dest]))?;
        }
        Ok(())
      }
    }
  }
}

impl Execute for ConditionalOperation {
  fn execute(&self, ctx: HashMap<String, String>) -> ProplateResult<()> {
    let true_ = self.conditions.iter().all(|c| c.eval_in_ctx(ctx.clone()));
    if true_ {
      for operation in &self.operations {
        operation.execute(ctx.clone())?;
      }
    }
    Ok(())
  }
}
