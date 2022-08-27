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
    #[error("An unknown error occurred.")]
    UnknownError(#[from] anyhow::Error),

    #[error("An unknown error occurred.")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Page not found.")]
    NotFound(),
}

impl<'r: 'o, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(
        self,
        request: &'r askama_rocket::Request<'_>,
    ) -> rocket::response::Result<'o> {
        match self {
            AppError::NotFound() => {}
            _ => {
                log::error!("{:#?}", self);
            }
        };

        let response = ErrorTemplate {
            description: self.to_string(),
        };

        (Status::Ok, response).respond_to(request)
    }
}
