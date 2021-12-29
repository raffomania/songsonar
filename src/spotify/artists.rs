use aspotify::{AlbumGroup, Artist, ArtistsAlbum, Client, Market};

use crate::get_all_pages;

pub async fn get_all_albums(
    client: &Client,
    artist: &Artist,
) -> Result<Vec<ArtistsAlbum>, aspotify::Error> {
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

    Ok(albums)
}
