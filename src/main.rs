#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use serde::Serialize;

pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

#[derive(Serialize)]
struct Recipe {
    id: i32,
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite()
        .manage(db_connection::init_pool())
        .mount(
            "/",
            routes![
                handlers::get_single_recipe,
                handlers::create_single_recipe,
                handlers::update_single_recipe,
            ],
        )
        .launch();
}
