use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::{FromRequest, Outcome},
    State,
};
use sqlx::Postgres;

use crate::basics::*;

pub struct RequestUri(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestUri {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestUri(request.uri().to_string()))
    }
}

pub struct Transaction<'request>(pub sqlx::Transaction<'request, Postgres>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Transaction<'r> {
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

        let tx = pool
            .begin()
            .await
            .map(|tx| Transaction(tx))
            .map_err(|e| AppError::from(e));
        tx.into_outcome(Status::InternalServerError)
    }
}
