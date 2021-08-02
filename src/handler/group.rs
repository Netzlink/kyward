use super::super::database::DbConn;
use super::super::diesel::prelude::*;
use super::super::models::group::Group;
use super::super::schema::groups::dsl::*;
use rocket::serde::json::Json;

#[get("/group")]
pub async fn list(db: DbConn) -> Json<Vec<Group>> {
  Json(
    db.run(|c| groups.load::<Group>(c).expect("Error loading groups"))
      .await,
  )
}

#[get("/group/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Group>> {
  Json(
    db.run(move |c| {
      groups
        .filter(id.eq(identifier))
        .load::<Group>(c)
        .expect("Error loading groups")
    })
    .await,
  )
}

#[post("/group", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Group>) -> Json<i32> {
  let new_group: Group = data.into_inner();
  let i = new_group.id;
  db.run(move |c| {
    diesel::insert_into(groups)
      .values(new_group)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[put("/group", format = "json", data = "<data>")]
pub async fn update(db: DbConn, data: Json<Group>) -> Json<i32> {
  let new_group: Group = data.into_inner();
  let i = new_group.id;
  db.run(move |c| diesel::update(groups).set(new_group).execute(c).unwrap())
    .await;
  Json(i)
}

#[delete("/group/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
  db.run(move |c| {
    diesel::delete(groups.filter(id.eq(identifier)))
      .execute(c)
      .unwrap()
  })
  .await;
  Json(identifier)
}
