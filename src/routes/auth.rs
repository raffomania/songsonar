use crate::basics::*;
use chrono::{Duration, Utc};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    uri,
};

use crate::{
    cookies,
    request_guards::{RequestUri, Transaction},
    schemas::users::User,
    spotify,
    storage::users::{fetch_user, insert_user, update_user},
};

#[get("/spotify-connected")]
pub async fn spotify_connected(
    cookies: &CookieJar<'_>,
    mut tx: Transaction<'_>,
    uri: RequestUri,
) -> Result<Redirect, AppError> {
    let url = uri.0;
    let url = format!(
        "{}{}",
        std::env::var("ROOT_URL")
            .context("Environment variable ROOT_URL not set")?,
        url
    );
    let state = cookies
        .get_private(cookies::OAUTH_STATE)
        .ok_or_else(|| anyhow!("missing state cookie"))?;
    let state = state.value();
    let client = spotify::get_client()?;
    client
        .redirected(&url, state)
        .await
        .context("Could not create spotify client")?;

    let spotify_id = client
        .users_profile()
        .get_current_user()
        .await
        .context("Couldn't fetch user")?
        .data
        .id;

    let access_token = client.current_access_token().await.0;
    let refresh_token = client
        .refresh_token()
        .await
        .ok_or_else(|| anyhow!("Expected to find a refresh token"))?;

    let existing_user = fetch_user(&mut tx, &spotify_id).await;

    let playlist_id = if let Some(id) = existing_user
        .as_ref()
        .ok()
        .and_then(|u| u.playlist_id.as_ref())
    {
        id.clone()
    } else {
        spotify::create_playlist(&client).await?
    };

    let user = match existing_user {
        Ok(user) => {
            update_user(
                &mut tx,
                &User {
                    spotify_id: spotify_id.clone(),
                    access_token,
                    refresh_token,
                    playlist_id: Some(playlist_id.clone()),
                    ..user
                },
            )
            .await?
        }
        Err(_) => {
            insert_user(
                &mut tx,
                User {
                    spotify_id: spotify_id.clone(),
                    playlist_id: Some(playlist_id.clone()),
                    access_token,
                    refresh_token,
                    weeks_in_playlist: Some(1),
                    can_read_private_playlists: None,
                },
            )
            .await?
        }
    };
    tx.0.commit().await?;

    // todo use revokable sessions for this
    let session = cookies::Session {
        spotify_id,
        expires: Utc::now() + Duration::days(7),
    };
    let mut session_cookie = Cookie::new(
        crate::cookies::SESSION,
        crate::cookies::Session::to_string(session),
    );
    session_cookie.set_secure(true);
    cookies.add_private(session_cookie);

    let weeks_in_playlist = user.weeks_in_playlist.unwrap_or(1);

    crate::spotify::update_playlist(&client, weeks_in_playlist, &playlist_id)
        .await?;

    Ok(Redirect::to(uri!(crate::routes::dashboard::dashboard())))
}
