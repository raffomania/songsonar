use askama::Template;
use rocket::http::ContentType;

#[get("/logo.svg")]
pub fn logo() -> (ContentType, &'static str) {
    let logo = include_str!("../../static/logo.svg");
    (ContentType::SVG, logo)
}

#[get("/favicon.ico")]
pub fn favicon() -> (ContentType, &'static str) {
    let logo = include_str!("../../static/favicon.svg");
    (ContentType::SVG, logo)
}

#[derive(Template)]
#[template(path = "privacy_policy.html")]
pub struct PrivacyPolicyTemplate {}

#[get("/privacy")]
pub fn privacy_policy() -> PrivacyPolicyTemplate {
    PrivacyPolicyTemplate {}
}
