#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate clap;

mod auth;
mod cors;
mod database;
mod schema;
mod v1alpha1;

use v1alpha1::handler::{company, door, group, person, token, ui, version};

// https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/
// TODO: https://docs.rs/rocket_oauth2/0.4.1/rocket_oauth2/struct.OAuth2.html
#[launch]
fn kyward() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .attach(database::DbConn::fairing())
        .attach(cors::CORS)
        .manage(auth::get_oauth_public_key(
            "https://login.microsoftonline.com/common/discovery/keys",
        ))
        .mount("/", routes![ui::index, ui::files, version::version])
        .mount(
            "/api/v1alpha1",
            routes![
                door::list,
                door::get,
                door::add,
                door::update,
                door::delete,
                group::list,
                group::get,
                group::add,
                group::update,
                group::delete,
                group::get_doors_by_group,
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
                company::list,
                company::get,
                company::add,
                company::update,
                company::delete,
            ],
        )
}
