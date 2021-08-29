use super::super::super::auth::User;
use super::super::super::database::DbConn;
use super::super::super::diesel::prelude::*;
use super::super::super::schema::doors::dsl::*;
use super::super::models::door::Door;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
#[get("/door")]
pub async fn list(db: DbConn, user: User) -> Result<Json<Vec<Door>>, Unauthorized<String>> {
    println!("{:#?}", user);
    Ok(Json(
        db.run(|c| doors.load::<Door>(c).expect("Error loading doors"))
            .await,
    ))
}

#[get("/door/<identifier>")]
pub async fn get(db: DbConn, identifier: i32) -> Json<Vec<Door>> {
    Json(
        db.run(move |c| {
            doors
                .filter(id.eq(identifier))
                .load::<Door>(c)
                .expect("Error loading doors")
        })
        .await,
    )
}

#[post("/door", format = "json", data = "<data>")]
pub async fn add(db: DbConn, data: Json<Door>) -> Json<i32> {
    let new_door: Door = data.into_inner();
    let i = new_door.id;
    db.run(move |c| {
        diesel::insert_into(doors)
            .values(new_door)
            .execute(c)
            .unwrap()
    })
    .await;
    Json(i)
}

#[put("/door", format = "json", data = "<data>")]
pub async fn update(db: DbConn, data: Json<Door>) -> Json<i32> {
    let new_door: Door = data.into_inner();
    let i = new_door.id;
    db.run(move |c| {
        diesel::update(doors.filter(id.eq(i)))
            .set(new_door)
            .execute(c)
            .unwrap()
    })
    .await;
    Json(i)
}

#[delete("/door/<identifier>")]
pub async fn delete(db: DbConn, identifier: i32) -> Json<i32> {
    db.run(move |c| diesel::delete(doors.find(identifier)).execute(c).unwrap())
        .await;
    Json(identifier)
}
