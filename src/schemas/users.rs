use rocket::{
    http::{Cookie, CookieJar},
    outcome::{try_outcome, IntoOutcome},
    request::FromRequest,
};
use sqlx::FromRow;

use crate::{cookies, db, storage};

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub spotify_id: String,
    pub playlist_id: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
    pub weeks_in_playlist: Option<i16>,
    pub can_read_private_playlists: Option<bool>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(
        request: &'r askama_rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let session = try_outcome!(request.guard::<cookies::Session>().await);

        let mut tx = try_outcome!(request
            .guard::<db::Transaction>()
            .await
            .map_failure(|(s, _)| (s, ())));

        let cookies: &CookieJar = try_outcome!(request
            .guard::<&CookieJar>()
            .await
            .map_failure(|(s, _)| (s, ())));

        storage::users::fetch_user(&mut tx, &session.spotify_id)
            .await
            .map_err(|_| {
                cookies.remove_private(Cookie::named(crate::cookies::SESSION));
            })
            .or_forward(())
    }
}
