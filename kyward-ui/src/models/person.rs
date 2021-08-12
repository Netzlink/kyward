use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub enabled: bool,
    pub ema: String,
    pub company_id: i32,
    pub token_id: i32,
    pub group_id: i32,
    pub description: String,
}
