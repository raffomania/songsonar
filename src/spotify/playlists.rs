use aspotify::{Client, PlaylistItemType};

use crate::basics::*;

pub async fn create_playlist(client: &Client) -> Result<String> {
    let name = "Song Sonar".to_string();
    let description = "New releases of artists you follow";

    #[cfg(debug_assertions)]
    let name = format!("{} dev", name);

    client
        .playlists()
        .create_playlist(&name, false, false, description)
        .await
        .context("Could not create playlist")
        .map(|res| res.data.id)
}

pub async fn replace_playlists_items(
    client: &Client,
    playlist_id: &str,
    playlist_items: Vec<String>,
) -> Result<()> {
    for (i, chunk) in playlist_items.chunks(100).enumerate() {
        let chunk = chunk
            .iter()
            .map(|id| PlaylistItemType::<&str, &str>::Track(id));

        if i == 0 {
            client
                .playlists()
                .replace_playlists_items(playlist_id, chunk)
                .await?;
        } else {
            client
                .playlists()
                .add_to_playlist(playlist_id, chunk, None)
                .await?;
        }
    }

    Ok(())
}
