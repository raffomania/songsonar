use aspotify::Client;
pub use auth::{get_authorization_url, get_client};

use crate::basics::*;

mod auth;

pub async fn update_playlist(client: Client) -> Result<()> {
    let mut followed_artists_page =
        client.follow().get_followed_artists(50, None).await?.data;
    let mut followed_artists = followed_artists_page.items;
    while let Some(ref after) = followed_artists_page.cursors.after {
        followed_artists_page = client
            .follow()
            .get_followed_artists(50, Some(after))
            .await?
            .data;
        followed_artists.append(&mut followed_artists_page.items);
    }

    log::debug!("Found {} artists", followed_artists.len());
    // log::debug!(
    //     "{:?}",
    //     followed_artists
    //         .iter()
    //         .map(|a| &a.name)
    //         .collect::<Vec<&String>>()
    // );

    Ok(())
}
