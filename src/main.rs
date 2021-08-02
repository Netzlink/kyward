#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use dotenv::dotenv;

// https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/
mod models;
mod handler;
use handler::door;
use handler::company;
use handler::group;
use handler::token;
use handler::person;
mod cors;
pub mod schema;
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
            door::list,
            door::get,
            door::add,
            door::update,
            door::delete,

            company::list,
            company::get,
            company::add,
            company::update,
            company::delete,

            group::list,
            group::get,
            group::add,
            group::update,
            group::delete,

            token::list,
            token::get,
            token::add,
            token::update,
            token::delete,

            person::list,
            person::get,
            person::add,
            person::update,
            person::delete,
        ])
        .attach(database::DbConn::fairing())
        .attach(cors::CORS)
}