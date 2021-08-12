use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Token {
    pub id: i32,
    pub value: String,
    pub reverse: String,
    pub description: String,
}
