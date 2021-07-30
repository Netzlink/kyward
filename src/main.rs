#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use dotenv::dotenv;

// https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/
mod models;
mod handler;
pub mod schema;
use handler::door;
mod database;
// Later yew endpoint
#[get("/")]
fn index() -> &'static str {
    "Hello from kyward!"
}



#[launch]
fn kyward() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![
            door::list_doors,
            door::get_door,
            door::add_door,
            door::update_door,
            door::delete_door,
        ])
        .attach(database::DbConn::fairing())
}