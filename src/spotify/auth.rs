use aspotify::{Client, Scope};
use rocket::http::{Cookie, CookieJar, SameSite};

use crate::basics::*;
use crate::cookies;

pub fn get_client() -> Result<Client> {
    let creds = aspotify::ClientCredentials::from_env().context(
        "Please provide CLIENT_ID and CLIENT_SECRET env vars with your spotify credentials."
    )?;

    Ok(Client::new(creds))
}

/// Get the authorization URL with a random state parameter.
/// If the state is not saved in a cookie yet, generate a new one
/// and persist it.
pub fn get_authorization_url(cookies: &CookieJar<'_>) -> Result<String> {
    let client_id = &get_client()?.credentials.id;
    let scopes = vec![
        Scope::UserFollowRead,
        Scope::PlaylistModifyPrivate,
        Scope::PlaylistReadPrivate,
        Scope::PlaylistModifyPublic,
    ];
    let force_approve = false;
    let root_url =
        std::env::var("ROOT_URL").expect("Please set the ROOT_URL env var");
    let redirect_uri = &format!("{}/spotify-connected", root_url);

    let previous_state = cookies
        .get_private(cookies::OAUTH_STATE)
        .map(|c| c.value().to_string());

    let url = if let Some(state) = previous_state {
        (
            aspotify::authorization_url_with_state(
                client_id,
                scopes,
                force_approve,
                redirect_uri,
                &state,
            ),
            state.to_string(),
        )
            .0
    } else {
        let (url, new_state) = aspotify::authorization_url(
            client_id,
            scopes,
            force_approve,
            redirect_uri,
        );

        // Persist new state as a cookie.
        let mut cookie = Cookie::new(cookies::OAUTH_STATE, new_state);
        // Allow reading this cookie when spotify redirects to us
        cookie.set_same_site(SameSite::Lax);
        cookies.add_private(cookie);

        url
    };

    Ok(url)
}
