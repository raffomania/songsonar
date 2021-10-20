use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::basics::*;

pub async fn create_db_pool() -> Result<Pool<Postgres>> {
    let database_url = std::env::var("DATABASE_URL")
        .context("Please set the DATABASE_URL environment variable.")?;

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .context("Could not create db pool")
}
