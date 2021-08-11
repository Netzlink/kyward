use super::super::super::database::DbConn;
use super::super::super::diesel::prelude::*;
use super::super::super::schema::companies::dsl::*;
use super::super::models::company::Company;
use rocket::serde::json::Json;

#[get("/company")]
pub async fn list(db: DbConn) -> Json<Vec<Company>> {
  Json(
    db.run(|c| {
      companies
        .load::<Company>(c)
        .expect("Error loading companies")
    })
    .await,
  )
}

#[get("/company/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Company>> {
  Json(
    db.run(move |c| {
      companies
        .filter(id.eq(identifier))
        .load::<Company>(c)
        .expect("Error loading companies")
    })
    .await,
  )
}

#[post("/company", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Company>) -> Json<i32> {
  let new: Company = data.into_inner();
  let i = new.id;
  db.run(move |c| {
    diesel::insert_into(companies)
      .values(new)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[put("/company", format = "json", data = "<data>")]
pub async fn update(db: DbConn, data: Json<Company>) -> Json<i32> {
  let new: Company = data.into_inner();
  let i = new.id;
  db.run(move |c| {
    diesel::update(companies.filter(id.eq(i)))
      .set(new)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[delete("/company/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
  db.run(move |c| {
    diesel::delete(companies.find(identifier))
      .execute(c)
      .unwrap()
  })
  .await;
  Json(identifier)
}
