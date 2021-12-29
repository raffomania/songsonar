use aspotify::{ArtistsAlbum, Client, Market, TrackSimplified};

use crate::get_all_pages;

pub async fn get_all_tracks(
    client: &Client,
    album: &ArtistsAlbum,
) -> Result<Vec<TrackSimplified>, aspotify::Error> {
    let tracks = get_all_pages!(offset, {
        client
            .albums()
            .get_album_tracks(&album.id, 50, offset, Some(Market::FromToken))
            .await?
            .data
    });

    Ok(tracks)
}
