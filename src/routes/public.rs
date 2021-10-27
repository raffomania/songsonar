use askama::Template;
use rocket::http::ContentType;

#[get("/favicon.ico")]
pub fn favicon_ico() -> (ContentType, &'static [u8]) {
    let icon = include_bytes!("../../static/favicon.ico");
    (ContentType::Icon, icon)
}

#[get("/favicon.svg")]
pub fn favicon_svg() -> (ContentType, &'static [u8]) {
    let icon = include_bytes!("../../static/favicon.svg");
    (ContentType::SVG, icon)
}

#[get("/logo.svg")]
pub fn logo_svg() -> (ContentType, &'static [u8]) {
    let logo = include_bytes!("../../static/logo.svg");
    (ContentType::SVG, logo)
}

#[derive(Template)]
#[template(path = "privacy_policy.html")]
pub struct PrivacyPolicyTemplate {}

#[get("/privacy")]
pub fn privacy_policy() -> PrivacyPolicyTemplate {
    PrivacyPolicyTemplate {}
}
