use askama::Template;
use rocket::http::CookieJar;

use crate::request_guards::LoggedInUser;

#[derive(Template)]
#[template(path = "logged_in.html")]
pub struct LoggedInTemplate {}

#[get("/")]
pub fn logged_in(_user: LoggedInUser) -> LoggedInTemplate {
    LoggedInTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    login_url: String,
}

#[get("/", rank = 1)]
pub fn index(cookies: &CookieJar<'_>) -> IndexTemplate {
    let login_url = crate::spotify::get_authorization_url(cookies);

    IndexTemplate { login_url }
}
