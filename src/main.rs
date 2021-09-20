#[macro_use]
extern crate rocket;

mod cookies;
mod errors;
mod request_guards;
mod routes;
mod spotify;

use routes::auth;
use routes::index::index;

#[launch]
fn rocket() -> _ {
    pretty_env_logger::init();
    rocket::build().mount("/", routes![index, auth::spotify_connected])
}
