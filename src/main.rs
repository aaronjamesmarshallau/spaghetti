#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod conversion;
pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

use rocket::Rocket;
use rocket::fairing::AdHoc;

use db_connection::Pool;

embed_migrations!();

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let maybe_pool = rocket.state::<Pool>();

    match maybe_pool {
        Some(pool) => match pool.get() {
            Ok(conn) => match embedded_migrations::run(&conn) {
                Ok(()) => Ok(rocket),
                Err(e) => {
                    println!("Failed to run database migrations: {}", e);
                    Err(rocket)
                }
            },
            Err(e) => {
                println!("Failed to get connection to run database migrations: {}", e);
                Err(rocket)
            },
        },
        None => {
            println!("Failed to retrieve database pool for migration step");
            Err(rocket)
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .manage(db_connection::init_pool())
        .mount(
            "/",
            routes![
                // Recipes
                handlers::recipes::get_single_recipe,
                handlers::recipes::create_single_recipe,
                handlers::recipes::update_single_recipe,
                handlers::recipes::archive_single_recipe,
                
                // Recipe Ingredients
                handlers::ingredients::get_recipe_ingredients,
                handlers::ingredients::create_recipe_ingredient,
                handlers::ingredients::update_recipe_ingredient,
                handlers::ingredients::archive_recipe_ingredient,

                // Ingredients
                handlers::ingredients::create_ingredient,
                handlers::ingredients::get_single_ingredient,
                handlers::ingredients::update_ingredient,
                handlers::ingredients::archive_ingredient,
            ],
        )
        .launch();
}
