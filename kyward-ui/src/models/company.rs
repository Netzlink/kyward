use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
}
