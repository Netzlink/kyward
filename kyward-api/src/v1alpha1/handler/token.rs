use super::super::super::database::DbConn;
use super::super::super::diesel::prelude::*;
use super::super::super::schema::tokens::dsl::*;
use super::super::models::token::Token;
use rocket::serde::json::Json;

#[get("/token")]
pub async fn list(db: DbConn) -> Json<Vec<Token>> {
  Json(
    db.run(|c| tokens.load::<Token>(c).expect("Error loading tokens"))
      .await,
  )
}

#[get("/token/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Token>> {
  Json(
    db.run(move |c| {
      tokens
        .filter(id.eq(identifier))
        .load::<Token>(c)
        .expect("Error loading tokens")
    })
    .await,
  )
}

#[post("/token", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Token>) -> Json<i32> {
  let new_token: Token = data.into_inner();
  let i = new_token.id;
  db.run(move |c| {
    diesel::insert_into(tokens)
      .values(new_token)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[put("/token", format = "json", data = "<data>")]
pub async fn update(db: DbConn, data: Json<Token>) -> Json<i32> {
  let new_token: Token = data.into_inner();
  let i = new_token.id;
  db.run(move |c| {
    diesel::update(tokens.filter(id.eq(i)))
      .set(new_token)
      .execute(c)
      .unwrap()
  })
  .await;
  Json(i)
}

#[delete("/token/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
  db.run(move |c| {
    diesel::delete(tokens.filter(id.eq(identifier)))
      .execute(c)
      .unwrap()
  })
  .await;
  Json(identifier)
}
