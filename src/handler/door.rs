use rocket::serde::json::Json;
use super::super::database::DbConn;
use super::super::models::door::Door;
use super::super::schema::doors::dsl::*;
use super::super::diesel::prelude::*;

#[get("/doors")]
pub async fn list_doors(db: DbConn) -> Json<Vec<Door>> {
    Json(
      db.run( |c| 
        doors.load::<Door>(c).expect("Error loading doors")
      ).await
    )
}