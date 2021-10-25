#[macro_use]
extern crate rocket;

mod args;
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

use args::{Command, StartOptions};
use db::MIGRATOR;

use crate::{args::Args, basics::*, db::create_db_pool};

#[rocket::main]
async fn main() -> Result<()> {
    init_logger_with_sentry();

    let _sentry = sentry::init();

    let args: Args = argh::from_env();

    match args.command.unwrap_or(Command::Start(StartOptions {})) {
        Command::Start(_) => start().await,
        Command::Migrate(_) => migrate().await,
    }
}

async fn migrate() -> Result<()> {
    let pool = create_db_pool().await?;

    MIGRATOR.run(&pool).await.context("Error during migration")
}

async fn start() -> Result<()> {
    let pool = create_db_pool().await?;

    let scheduled_updates =
        tokio::spawn(async { schedule::schedule_updates().await });

    let web_server = rocket::build()
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
        .launch();

    tokio::select! {
        res = scheduled_updates => res?,
        res = web_server => res.context("Could not start rocket server"),
    }
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
