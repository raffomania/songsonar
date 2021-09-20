use rocket::{http::Status, response::Responder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unknown error")]
    UnknownError(#[from] anyhow::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(
        self,
        request: &'r askama_rocket::Request<'_>,
    ) -> rocket::response::Result<'o> {
        let response = match self {
            AppError::UnknownError(e) => {
                log::error!("{:?}", e);

                "An unknown error occured."
            }
        };

        (Status::Ok, response).respond_to(request)
    }
}
