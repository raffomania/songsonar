#[macro_use]
extern crate rocket;

mod basics;
mod cookies;
mod errors;
mod request_guards;
mod routes;
mod schemas;
mod spotify;
mod storage;

use crate::basics::*;
use routes::auth;
use routes::index::index;
use sqlx::postgres::PgPoolOptions;

#[rocket::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Please set the DATABASE_URL environment variable.");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                auth::spotify_connected,
                routes::public::styles,
                routes::public::logo,
                routes::public::favicon
            ],
        )
        .manage(pool)
        .launch()
        .await
        .context("Could not launch rocket")
}
