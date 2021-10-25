use crate::storage;
use askama::Template;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;

use crate::basics::*;
use crate::request_guards::LoggedInUser;
use crate::request_guards::Transaction;

#[get("/dashboard", rank = 1)]
pub fn not_logged_in() -> Redirect {
    Redirect::to(uri!("/"))
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {}

#[get("/dashboard")]
pub async fn dashboard(
    mut tx: Transaction<'_>,
    session: LoggedInUser,
    cookies: &CookieJar<'_>,
) -> Result<DashboardTemplate, Redirect> {
    storage::users::fetch_user(&mut tx, &session.0.spotify_id)
        .await
        .map_err(|_| {
            cookies.remove_private(Cookie::named(crate::cookies::SESSION));
            Redirect::to(uri!("/"))
        })?;

    Ok(DashboardTemplate {})
}
