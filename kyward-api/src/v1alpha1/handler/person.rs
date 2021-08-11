use super::super::super::database::DbConn;
use super::super::super::diesel::prelude::*;
use super::super::models::person::Person;
use super::super::super::schema::persons::dsl::*;
use rocket::serde::json::Json;

#[get("/person")]
pub async fn list(db: DbConn) -> Json<Vec<Person>> {
  Json(
    db.run(|c| persons.load::<Person>(c).expect("Error loading persons"))
      .await,
  )
}

#[get("/person/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Person>> {
  Json(
    db.run(move |c| {
      persons
        .filter(id.eq(identifier))
        .load::<Person>(c)
        .expect("Error loading persons")
    })
    .await,
  )
}

#[post("/person", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Person>) -> Json<i32> {
  let new_person: Person = data.into_inner();
  let i = new_person.id;
  db.run(move |c| {
    diesel::insert_into(persons)
      .values(new_person)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[put("/person", format = "json", data = "<data>")]
pub async fn update(db: DbConn, data: Json<Person>) -> Json<i32> {
  let new_person: Person = data.into_inner();
  let i = new_person.id;
  db.run(move |c| {
    diesel::update(persons.filter(id.eq(i)))
      .set(new_person)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[delete("/person/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
  db.run(move |c| {
    diesel::delete(persons.filter(id.eq(identifier)))
      .execute(c)
      .unwrap()
  })
  .await;
  Json(identifier)
}
