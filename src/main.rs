#[macro_use]
extern crate rocket;

mod basics;
mod cookies;
mod errors;
mod request_guards;
mod routes;
mod schedule;
mod schemas;
mod sentry;
mod spotify;
mod storage;

use crate::basics::*;
use sqlx::postgres::PgPoolOptions;

#[rocket::main]
async fn main() -> Result<()> {
    init_logger_with_sentry();

    let _sentry = sentry::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Please set the DATABASE_URL environment variable.");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    tokio::spawn(async {
        schedule::schedule_updates().await;
    });

    rocket::build()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::index::logged_in,
                routes::auth::spotify_connected,
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

/// Create a new logger instance, wrap it with a sentry listener and set the
/// wrapper as the new global logging instance.
fn init_logger_with_sentry() {
    let mut log_builder = pretty_env_logger::formatted_builder();

    log_builder.parse_filters(
        &std::env::var("RUST_LOG")
            .expect("Please set the RUST_LOG environment variable"),
    );
    let logger = log_builder.build();
    let max_level = logger.filter();

    let sentry_logger = sentry::get_logger(logger);

    log::set_boxed_logger(Box::new(sentry_logger))
        .expect("Could not initialize logger");
    log::set_max_level(max_level);
}
