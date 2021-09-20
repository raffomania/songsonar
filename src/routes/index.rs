use askama::Template;
use rocket::http::CookieJar;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    login_url: String,
}

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> IndexTemplate {
    let login_url = crate::spotify::get_authorization_url(cookies);

    IndexTemplate { login_url }
}
