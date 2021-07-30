use rocket::serde::{Deserialize, Serialize};

use super::door::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub doors: Vec<Door>,
    pub description: String,
}