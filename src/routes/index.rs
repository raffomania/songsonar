use askama::Template;
use rocket::{http::CookieJar, response::Redirect};

use crate::request_guards::LoggedInUser;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    login_url: String,
}

#[get("/")]
pub fn logged_in_index(_user: LoggedInUser) -> Redirect {
    Redirect::to(uri!(crate::routes::dashboard::dashboard()))
}

#[get("/", rank = 1)]
pub fn index(cookies: &CookieJar<'_>) -> IndexTemplate {
    let login_url = crate::spotify::get_authorization_url(cookies);

    IndexTemplate { login_url }
}
