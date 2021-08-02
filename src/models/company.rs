use rocket::serde::{Deserialize, Serialize};
use super::super::schema::companies;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[serde(crate = "rocket::serde")]
#[table_name="companies"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
}
