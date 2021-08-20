use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub family_name: String,
    pub given_name: String,
    pub unique_name: String,
}
