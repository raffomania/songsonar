use crate::{basics::*, request_guards::Transaction, schemas::users::User};
use sqlx::query_as;

pub async fn insert_user(tx: &mut Transaction<'_>, user: User) -> Result<User> {
    let user = query_as!(
        User,
        r#"insert into users
        (spotify_id, playlist_id, access_token, refresh_token, weeks_in_playlist)
        values ($1, $2, $3, $4, $5)
        returning *
    "#,
    user.spotify_id,
    user.playlist_id,
    user.access_token,
    user.refresh_token,
    user.weeks_in_playlist
    ).fetch_one(&mut tx.0).await?;

    Ok(user)
}

pub async fn fetch_user(
    tx: &mut Transaction<'_>,
    spotify_id: &str,
) -> Result<User> {
    let user = query_as!(
        User,
        r#"select * from users
        where spotify_id = $1
    "#,
        spotify_id,
    )
    .fetch_one(&mut tx.0)
    .await?;

    Ok(user)
}

pub async fn update_user(
    tx: &mut Transaction<'_>,
    user: &User,
) -> Result<User> {
    let user = query_as!(
        User,
        r#"update users
        set playlist_id = $2, access_token = $3, refresh_token = $4, weeks_in_playlist = $5
        where spotify_id = $1
        returning *
    "#,
    user.spotify_id,
    user.playlist_id,
    user.access_token,
    user.refresh_token,
    user.weeks_in_playlist
    ).fetch_one(&mut tx.0).await?;

    Ok(user)
}
