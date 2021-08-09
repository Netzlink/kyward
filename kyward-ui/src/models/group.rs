use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    PartialEq,
    Clone,
    Deserialize,
    Serialize,
)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub door_id: i32,
    pub description: String,
}
