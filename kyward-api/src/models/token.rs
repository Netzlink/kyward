use super::super::schema::tokens;
use rocket::serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Deserialize,
    Serialize,
    Queryable,
    Insertable,
    Identifiable,
    AsChangeset,
    Associations,
)]
#[serde(crate = "rocket::serde")]
#[table_name = "tokens"]
pub struct Token {
    pub id: i32,
    pub value: String,
    pub reverse: String,
    pub description: String,
}
