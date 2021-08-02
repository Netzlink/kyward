use rocket::serde::{Deserialize, Serialize};
use super::super::schema::groups;
use super::door::Door;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Door)]
#[table_name="groups"]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub door_id: i32,
    pub description: String,
}