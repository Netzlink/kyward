use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::group::*;
use super::token::*;
use super::company::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub company: Company,
    pub token: Token,
    pub group: Group,
    pub labels: HashMap<String, String>,
    pub description: String,
}