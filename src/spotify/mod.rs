use aspotify::{AlbumGroup, Artist, Client, Market};
pub use auth::{get_authorization_url, get_client};
use chrono::{Duration, Utc};

use crate::{basics::*, get_all_cursor_pages, get_all_pages};

mod auth;
mod playlist;
mod util;

pub use playlist::create_playlist;

pub async fn update_playlist(
    client: Client,
    weeks_in_playlist: i16,
    playlist_id: &str,
) -> Result<()> {
    let followed_artists: Vec<Artist> = get_all_cursor_pages!(after, {
        client.follow().get_followed_artists(50, after).await?.data
    });

    log::debug!("Found {} artists", followed_artists.len());

    let mut track_ids: Vec<String> = Vec::new();
    for artist in followed_artists {
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

        let relevant_albums =
            albums.iter().filter(|a| a.release_date >= cutoff);

        for album in relevant_albums {
            log::debug!("Found album '{}'", &album.name);
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

            let mut new_track_ids: Vec<String> = tracks
                .into_iter()
                .filter_map(|track| {
                    track.linked_from.map(|link| link.id).or(track.id)
                })
                .collect();

            track_ids.append(&mut new_track_ids);
        }
    }

    log::debug!("Found {} tracks", track_ids.len());

    playlist::replace_playlists_items(&client, playlist_id, track_ids).await?;

    Ok(())
}
