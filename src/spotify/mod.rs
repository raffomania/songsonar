use aspotify::{Artist, ArtistsAlbum, Client};
use chrono::{Duration, Utc};
use rocket::futures::future::join_all;

use crate::{basics::*, get_all_cursor_pages};

mod albums;
mod artists;
mod auth;
mod playlists;
mod util;

pub use auth::{get_authorization_url, get_client};
pub use playlists::create_playlist;

pub async fn update_playlist(
    client: &Client,
    weeks_in_playlist: i16,
    playlist_id: &str,
) -> Result<()> {
    log::info!("Updating playlist {}", playlist_id);
    let followed_artists: Vec<Artist> = get_all_cursor_pages!(after, {
        client.follow().get_followed_artists(50, after).await?.data
    });

    log::debug!("Found {} artists", followed_artists.len());

    let album_futures = followed_artists.iter().map(|artist| async move {
        let cutoff = Utc::now().naive_local().date()
            - Duration::weeks(weeks_in_playlist.into());

        Ok(artists::get_all_albums(client, artist)
            .await?
            .into_iter()
            .filter(|a| a.release_date >= cutoff)
            .collect())
    });
    // Check that all entries are actually Ok()
    let all_albums: Result<Vec<Vec<ArtistsAlbum>>, anyhow::Error> =
        join_all(album_futures).await.into_iter().collect();

    // Flatten vectors
    let mut all_albums: Vec<ArtistsAlbum> =
        all_albums?.into_iter().flatten().collect();

    log::info!(
        "Found {} albums for the last {} weeks",
        all_albums.len(),
        weeks_in_playlist
    );

    // Reverse-sort by release date (newest first)
    all_albums.sort_by(|a, b| b.release_date.cmp(&a.release_date));

    let track_id_futures = all_albums.iter().map(|album| async move {
        log::debug!("Found album '{}', {}", &album.name, album.release_date);

        let new_track_ids: Result<Vec<String>, anyhow::Error> =
            albums::get_all_tracks(client, album)
                .await?
                .into_iter()
                .map(|track| {
                    track
                        .linked_from
                        .map(|link| link.id)
                        .or(track.id)
                        // We assume that track IDs are always present, because they are only missing for local files which we should never encounter
                        .ok_or_else(|| anyhow!("missing track ID"))
                })
                .collect();

        new_track_ids
    });

    // Check that each vector of an album's tracks is Ok()
    let track_ids: Result<Vec<_>, anyhow::Error> =
        join_all(track_id_futures).await.into_iter().collect();

    // Flatten all track ids
    let track_ids: Vec<String> = track_ids?.into_iter().flatten().collect();

    log::info!("Found {} tracks", track_ids.len());

    playlists::replace_playlists_items(client, playlist_id, track_ids).await?;

    log::info!("Playlist {} updated", playlist_id);

    Ok(())
}
