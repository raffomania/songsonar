use crate::{basics::*, db::Transaction, schemas::users::User};
use sqlx::{query, query_as};

pub async fn insert_user(tx: &mut Transaction, user: User) -> Result<User> {
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
    tx: &mut Transaction,
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

pub async fn list_users(tx: &mut Transaction) -> Result<Vec<User>> {
    query_as!(User, r#"select * from users"#)
        .fetch_all(&mut tx.0)
        .await
        .context("Could not fetch users")
}

pub async fn update_user(tx: &mut Transaction, user: &User) -> Result<User> {
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

pub async fn delete_user(tx: &mut Transaction, user: User) -> Result<()> {
    let affected_rows = query!(
        r#"delete from users
        where spotify_id = $1"#,
        user.spotify_id
    )
    .execute(&mut tx.0)
    .await?
    .rows_affected();

    if affected_rows != 1 {
        return Err(anyhow!(
            "Unexpected amount of users deleted: {}",
            affected_rows
        ));
    }

    Ok(())
}
