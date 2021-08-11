use super::super::super::database::DbConn;
use super::super::super::diesel::prelude::*;
use super::super::models::door::Door;
use super::super::models::group::Group;
use super::super::super::schema::*;
use rocket::serde::json::Json;

#[get("/group")]
pub async fn list(db: DbConn) -> Json<Vec<Group>> {
  Json(
    db.run(|c| groups::table.load::<Group>(c).expect("Error loading groups"))
      .await,
  )
}

#[get("/group/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Group>> {
  Json(
    db.run(move |c| {
      groups::table
        .filter(groups::id.eq(identifier))
        .load::<Group>(c)
        .expect("Error loading groups")
    })
    .await,
  )
}

#[get("/group/<identifier>/doors")]
pub async fn get_doors_by_group(db: DbConn, identifier: i32) -> Json<Vec<Door>> {
  let mut doors: Vec<Door> = vec![];
  let result = db.run(move |c| {
    groups::table
      .inner_join(doors::table)
      .filter(groups::id.eq(identifier))
      .load::<(Group, Door)>(c)
      .expect("Error loading groups")
  }).await;
  for tup in result.iter() {
    let (_, door) = tup;
    doors.push(door.clone());
  }
  Json(doors)
}

#[post("/group", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Group>) -> Json<i32> {
  let new_group: Group = data.into_inner();
  let i = new_group.id;
  db.run(move |c| {
    diesel::insert_into(groups::table)
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
  db.run(move |c| {
    diesel::update(groups::table.filter(groups::id.eq(i)))
      .set(new_group)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[delete("/group/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
  db.run(move |c| {
    diesel::delete(groups::table.filter(groups::id.eq(identifier)))
      .execute(c)
      .unwrap()
  })
  .await;
  Json(identifier)
}