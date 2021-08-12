use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub compartment: String,
    pub level: String,
    pub building: String,
    pub description: String,
}
