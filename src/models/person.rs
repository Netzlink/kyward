use rocket::serde::{Deserialize, Serialize};
use super::super::schema::persons;
use super::group::Group;
use super::company::Company;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[serde(crate = "rocket::serde")]
#[table_name="persons"]
#[belongs_to(Group)]
#[belongs_to(Company)]
#[belongs_to(Token)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub company_id: i32,
    pub token_id: i32,
    pub group_id: i32,
    pub description: String,
}