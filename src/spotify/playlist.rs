use aspotify::{Client, PlaylistItemType};

use crate::basics::*;

pub async fn create_playlist(client: &Client) -> Result<String> {
    client
        .playlists()
        .create_playlist("Song Sonar dev", false, false, "")
        .await
        .context("Could not create playlist")
        .map(|res| res.data.id)
}

pub async fn replace_playlists_items(
    client: &Client,
    playlist_id: &str,
    playlist_items: Vec<String>,
) -> Result<()> {
    for chunk in playlist_items.chunks(100) {
        let chunk = chunk
            .iter()
            .map(|id| PlaylistItemType::<&str, &str>::Track(id));
        client
            .playlists()
            .replace_playlists_items(playlist_id, chunk)
            .await?;
    }

    Ok(())
}
