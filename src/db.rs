use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::FromRequest,
    request::Outcome,
    State,
};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};

use crate::basics::*;

pub static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn create_db_pool() -> Result<Pool<Postgres>> {
    let database_url = std::env::var("DATABASE_URL")
        .context("Please set the DATABASE_URL environment variable.")?;

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .context("Could not create db pool")
}

pub struct Transaction(pub sqlx::Transaction<'static, Postgres>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Transaction {
    type Error = AppError;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let pool = try_outcome!(request
            .guard::<&State<sqlx::PgPool>>()
            .await
            .map_failure(|_| (
                Status::InternalServerError,
                anyhow!("Failed to get DB pool").into()
            )));

        let tx = pool.begin().await.map(Transaction).map_err(AppError::from);
        tx.into_outcome(Status::InternalServerError)
    }
}
