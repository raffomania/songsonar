use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    hello: String,
}

#[get("/")]
pub fn index() -> IndexTemplate {
    IndexTemplate {
        hello: "Hello world!".to_string(),
    }
}
