use serde::{Deserialize, Serialize};

pub mod adapter;

#[derive(Serialize, Deserialize, Debug)]
pub enum JSONArgType {
    Text,
    Select,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONArg {
    pub key: String,
    pub q_type: JSONArgType,
    pub label: String,
    pub is_required: Option<bool>,
    pub options: Option<Vec<String>>,
}
