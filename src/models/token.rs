use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub id: i64,
    pub token: String,
    pub reverse_token: String,
    pub description: String, 
}