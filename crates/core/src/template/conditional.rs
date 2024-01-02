use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum StringCompareOp {
  Eq,
  NotEqual,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
  lhs: String,
  op: StringCompareOp,
  rhs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
  Copy { files: Vec<String>, dest: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConditionalOperation {
  conditions: Vec<Condition>,
  /// Actions to execute if the above conditions are true
  actions: Vec<Action>,
}
