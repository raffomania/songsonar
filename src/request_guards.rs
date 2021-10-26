use rocket::request::{FromRequest, Outcome};

pub struct RequestUri(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestUri {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestUri(request.uri().to_string()))
    }
}
