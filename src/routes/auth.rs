use anyhow::{anyhow, Context};
use askama::Template;
use rocket::http::CookieJar;

use crate::{cookies, errors::AppError, request_guards::RequestUri, spotify};

#[derive(Template)]
#[template(path = "logged_in.html")]
pub struct LoggedInTemplate {}

#[get("/spotify-connected")]
pub async fn spotify_connected(
    cookies: &CookieJar<'_>,
    uri: RequestUri,
) -> Result<LoggedInTemplate, AppError> {
    let url = uri.0;
    let url = format!(
        "{}{}",
        std::env::var("ROOT_URL")
            .context("Environment variable ROOT_URL not set")?,
        url
    );
    let state = cookies
        .get_private(cookies::OAUTH_STATE)
        .ok_or(anyhow!("missing state cookie"))?;
    let state = state.value();
    let _client = spotify::get_client()
        .redirected(&url, state)
        .await
        .context("Could not create spotify client")?;
    // TODO save refresh token
    Ok(LoggedInTemplate {})
}
