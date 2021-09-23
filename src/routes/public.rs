use rocket::http::ContentType;

#[get("/main.css")]
pub fn styles() -> (ContentType, &'static str) {
    let styles = include_str!("../../static/main.css");
    (ContentType::CSS, styles)
}

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
