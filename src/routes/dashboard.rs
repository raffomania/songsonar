use askama::Template;
use rocket::response::Redirect;

use crate::basics::*;
use crate::request_guards::LoggedInUser;

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
    mut tx: crate::request_guards::Transaction<'_>,
) -> Result<Redirect, AppError> {
    use crate::schemas::users::User;

    let client = crate::spotify::get_client()?;
    let user =
        crate::storage::users::fetch_user(&mut tx, &session.0.spotify_id)
            .await?;
    client
        .set_refresh_token(Some(user.refresh_token.clone()))
        .await;

    let playlist_id = if let Some(id) = user.playlist_id {
        id
    } else {
        let playlist_id = crate::spotify::create_playlist(&client).await?;
        crate::storage::users::update_user(
            &mut tx,
            &User {
                playlist_id: Some(playlist_id.clone()),
                ..user
            },
        )
        .await?;
        playlist_id
    };

    tx.0.commit().await?;

    crate::spotify::update_playlist(
        &client,
        user.weeks_in_playlist.unwrap_or(1),
        &playlist_id,
    )
    .await?;
    Ok(Redirect::to(uri!(dashboard())))
}

#[cfg(not(debug_assertions))]
#[get("/dashboard/update_playlist")]
pub async fn update_playlist(
    _session: LoggedInUser,
) -> Result<Redirect, AppError> {
    Ok(Redirect::to(uri!(dashboard())))
}
