use aspotify::{AlbumGroup, Artist, ArtistsAlbum, Client, Market};
use chrono::{Duration, Utc};
use rocket::futures::future::join_all;

use crate::{basics::*, get_all_cursor_pages, get_all_pages};

mod auth;
mod playlist;
mod util;

pub use auth::{get_authorization_url, get_client};
pub use playlist::create_playlist;

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
        let albums = get_all_pages!(offset, {
            client
                .artists()
                .get_artist_albums(
                    &artist.id,
                    Some(&[AlbumGroup::Single, AlbumGroup::Album]),
                    50,
                    offset,
                    Some(Market::FromToken),
                )
                .await?
                .data
        });

        let cutoff = Utc::now().naive_local().date()
            - Duration::weeks(weeks_in_playlist.into());

        Result::<Vec<_>, anyhow::Error>::Ok(
            albums
                .into_iter()
                .filter(|a| a.release_date >= cutoff)
                .collect(),
        )
    });
    // Check that all entries are actually Ok()
    let all_albums: Result<Vec<_>, _> =
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
        let tracks = get_all_pages!(offset, {
            client
                .albums()
                .get_album_tracks(
                    &album.id,
                    50,
                    offset,
                    Some(Market::FromToken),
                )
                .await?
                .data
        });

        let new_track_ids: Vec<String> = tracks
            .into_iter()
            .filter_map(|track| {
                track.linked_from.map(|link| link.id).or(track.id)
            })
            .collect();

        Ok(new_track_ids)
    });

    // Check that each vector of an album's tracks is Ok()
    let track_ids: Result<Vec<_>, anyhow::Error> =
        join_all(track_id_futures).await.into_iter().collect();

    // Flatten all track ids
    let track_ids: Vec<String> = track_ids?.into_iter().flatten().collect();

    log::info!("Found {} tracks", track_ids.len());

    playlist::replace_playlists_items(client, playlist_id, track_ids).await?;

    log::info!("Playlist {} updated", playlist_id);

    Ok(())
}
