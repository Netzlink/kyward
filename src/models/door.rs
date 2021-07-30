use rocket::serde::{Deserialize, Serialize};
use super::super::schema::doors;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="doors"]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub compartment: String,
    pub level: String,
    pub building: String,
    pub description: String,
}