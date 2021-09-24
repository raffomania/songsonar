use askama::Template;
use rocket::{http::Status, response::Responder};
use thiserror::Error;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    description: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unknown error")]
    UnknownError(#[from] anyhow::Error),

    #[error("Unknown error")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(
        self,
        request: &'r askama_rocket::Request<'_>,
    ) -> rocket::response::Result<'o> {
        log::error!("{:#?}", self);

        let response = ErrorTemplate {
            description: "An unknown error occurred.".to_string(),
        };

        (Status::Ok, response).respond_to(request)
    }
}
