use std::iter;

use crate::basics::*;
use chrono::{DateTime, Duration, Utc};
use miniserde::{Deserialize, Serialize};
use rand::Rng;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::{try_outcome, IntoOutcome};
use rocket::request::{FromRequest, Outcome};

pub static OAUTH_STATE: &str = "oauth_state";
pub static SESSION: &str = "session";

#[derive(Serialize, Deserialize, Debug)]
struct JsonSession {
    pub spotify_id: String,
    pub expires: String,
    pub csrf_token: String,
}

#[derive(Debug)]
pub struct Session {
    pub spotify_id: String,
    pub expires: DateTime<Utc>,
    pub csrf_token: String,
}

impl Session {
    pub fn new(spotify_id: String) -> Session {
        let mut rng = rand::thread_rng();
        let csrf_token: String = iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .take(32)
            .collect();

        Session {
            spotify_id,
            expires: Utc::now() + Duration::days(7),
            csrf_token,
        }
    }

    pub fn from_str(input: &str) -> Result<Session> {
        let deserialized: JsonSession = miniserde::json::from_str(input)?;
        let expires =
            chrono::DateTime::parse_from_rfc3339(&deserialized.expires)?
                .with_timezone(&Utc);

        Ok(Session {
            expires,
            spotify_id: deserialized.spotify_id,
            csrf_token: deserialized.csrf_token,
        })
    }

    pub fn into_cookie(self) -> Cookie<'static> {
        let expires = self.expires.to_rfc3339();
        let json_session = &JsonSession {
            spotify_id: self.spotify_id,
            csrf_token: self.csrf_token,
            expires,
        };

        let mut cookie =
            Cookie::new(SESSION, miniserde::json::to_string(json_session));
        cookie.set_secure(true);

        cookie
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let cookies: &CookieJar = try_outcome!(request
            .guard::<&CookieJar>()
            .await
            .map_failure(|_| (Status::InternalServerError, ())));

        let maybe_session: Option<Session> = cookies
            .get_private(SESSION)
            .and_then(|s| Session::from_str(s.value()).ok())
            .filter(|s: &Session| s.expires > Utc::now());

        if maybe_session.is_none() {
            cookies.remove_private(Cookie::named(SESSION));
        }

        maybe_session.or_forward(())
    }
}
