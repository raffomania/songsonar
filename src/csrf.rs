use rocket::request::FromRequest;

use crate::{basics::*, cookies};

pub struct Validation {
    expected_token: String,
}

#[derive(FromForm)]
pub struct Form {
    pub csrf_token: String,
}

impl Validation {
    pub fn validate(&self, token: &str) -> Result<()> {
        ring::constant_time::verify_slices_are_equal(
            self.expected_token.as_bytes(),
            token.as_bytes(),
        )
        .map_err(|_| {
            anyhow!(
                "Invalid CSRF token, expected: {}, actual: {}",
                self.expected_token,
                token
            )
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Validation {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        request
            .guard::<cookies::Session>()
            .await
            .map_failure(|(s, _)| (s, ()))
            .map(|session| Validation {
                expected_token: session.csrf_token,
            })
    }
}
