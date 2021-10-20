use askama::Template;
use rocket::response::Redirect;

use crate::request_guards::{LoggedInUser, Transaction};
use crate::spotify;
use crate::{basics::*, storage};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {}

#[cfg(not(debug_assertions))]
#[get("/dashboard")]
pub fn dashboard(_user: LoggedInUser) -> DashboardTemplate {
    DashboardTemplate {}
}

#[derive(Template)]
#[template(path = "debug_dashboard.html")]
pub struct DebugDashboardTemplate {}

#[cfg(debug_assertions)]
#[get("/dashboard")]
pub fn dashboard(_user: LoggedInUser) -> DebugDashboardTemplate {
    DebugDashboardTemplate {}
}

#[cfg(debug_assertions)]
#[get("/dashboard/update_playlist")]
pub async fn update_playlist(
    session: LoggedInUser,
    mut tx: Transaction<'_>,
) -> Result<Redirect, AppError> {
    let client = crate::spotify::get_client()?;
    let user =
        storage::users::fetch_user(&mut tx, &session.0.spotify_id).await?;
    client
        .set_refresh_token(Some(user.refresh_token.clone()))
        .await;

    let playlist_id = if let Some(id) = user.playlist_id {
        id
    } else {
        spotify::create_playlist(&client).await?
    };

    spotify::update_playlist(
        client,
        user.weeks_in_playlist.unwrap_or(1),
        &playlist_id,
    )
    .await?;
    Ok(Redirect::to(uri!(dashboard())))
}

#[cfg(not(debug_assertions))]
#[get("/dashboard/update_playlist")]
pub async fn update_playlist(
    session: LoggedInUser,
) -> Result<Redirect, AppError> {
    Ok(Redirect::to(uri!(dashboard())))
}
