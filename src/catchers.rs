use rocket::Request;

use crate::basics::AppError;

#[catch(404)]
pub fn not_found(_req: &Request) -> AppError {
    AppError::NotFound()
}
