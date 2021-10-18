use chrono::Utc;
use rocket::{
    http::{CookieJar, Status},
    outcome::{try_outcome, IntoOutcome},
    request::{FromRequest, Outcome},
    State,
};
use sqlx::Postgres;

use crate::{
    basics::*,
    cookies::{self, Session},
};

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

#[derive(Debug)]
pub struct LoggedInUser(pub Session);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let cookies = try_outcome!(request
            .guard::<&CookieJar>()
            .await
            .map_failure(|_| (Status::InternalServerError, ())));

        let maybe_user: Option<LoggedInUser> = cookies
            .get_private(cookies::SESSION)
            .and_then(|s| cookies::Session::from_str(s.value()).ok())
            .filter(|s: &Session| s.expires > Utc::now())
            .map(LoggedInUser);

        maybe_user.or_forward(())
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

        let tx = pool.begin().await.map(Transaction).map_err(AppError::from);
        tx.into_outcome(Status::InternalServerError)
    }
}
