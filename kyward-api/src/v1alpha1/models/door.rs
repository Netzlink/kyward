use super::super::super::schema::doors;
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
#[table_name = "doors"]
pub struct Door {
    pub id: i32,
    pub name: String,
    pub compartment: String,
    pub level: String,
    pub building: String,
    pub description: String,
}
