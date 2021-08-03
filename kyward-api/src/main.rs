#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rust_embed;

mod cors;
mod database;
mod handler;
mod models;
mod schema;

use handler::{company, door, group, person, token, ui};

// https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/
// TODO: https://docs.rs/rocket_oauth2/0.4.1/rocket_oauth2/struct.OAuth2.html
#[launch]
fn kyward() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .mount("/", routes![ui::index, ui::files])
        .mount(
            "/api",
            routes![
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
            ],
        )
        .attach(database::DbConn::fairing())
        .attach(cors::CORS)
}
