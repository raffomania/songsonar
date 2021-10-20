#[macro_use]
extern crate rocket;

mod basics;
mod cookies;
mod db;
mod errors;
mod request_guards;
mod routes;
mod schedule;
mod schemas;
mod sentry;
mod spotify;
mod storage;

use crate::{basics::*, db::create_db_pool};

#[rocket::main]
async fn main() -> Result<()> {
    init_logger_with_sentry();

    let _sentry = sentry::init();

    let pool = create_db_pool().await.unwrap();

    tokio::spawn(async {
        schedule::schedule_updates().await;
    });

    rocket::build()
        .mount(
            "/",
            routes![
                routes::index::index,
                routes::index::logged_in_index,
                routes::dashboard::dashboard,
                routes::dashboard::update_playlist,
                routes::auth::spotify_connected,
                routes::public::styles,
                routes::public::logo,
                routes::public::favicon,
                routes::public::privacy_policy
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
