use rocket::serde::json::Json;
use super::super::database::DbConn;
use super::super::models::door::Door;
use super::super::schema::doors::dsl::*;
use super::super::diesel::prelude::*;

#[get("/door")]
pub async fn list_doors(db: DbConn) -> Json<Vec<Door>> {
    Json(
      db.run( |c| 
        doors
          .load::<Door>(c)
          .expect("Error loading doors")
      ).await
    )
}

#[get("/door/<identifier>")]
pub async fn get_door(db: DbConn, identifier: i32) -> Json<Vec<Door>> {
    Json(
      db.run( move |c| 
        doors.filter(
            id.eq(identifier)
        )
          .load::<Door>(c)
          .expect("Error loading doors")
      ).await
    )
}

#[post("/door", format = "json", data = "<data>")]
pub async fn add_door(db: DbConn, data: Json<Door>) -> Json<i32> {
    let new_door: Door = data.into_inner();
    let i = new_door.id;
    db.run(move |c| 
        diesel::insert_into(doors)
          .values(new_door)
          .execute(c).unwrap()
    ).await;
    Json(i)
}

#[put("/door/<identifier>", format = "json", data = "<data>")]
pub async fn update_door(db: DbConn, identifier: i32, data: Json<Door>) -> Json<i32> {
    let new_door: Door = data.into_inner();
    let i = new_door.id;
    db.run(move |c| 
        diesel::update(doors.find(identifier))
          .set(description.eq(new_door.description))
          .execute(c).unwrap()
    ).await;
    Json(i)
}

#[delete("/door/<identifier>")]
pub async fn delete_door(db: DbConn, identifier: i32) -> Json<i32> {
    db.run(move |c| 
        diesel::delete(doors.find(identifier))
          .execute(c).unwrap()
    ).await;
    Json(identifier)
}