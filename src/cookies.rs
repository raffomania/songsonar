use chrono::{DateTime, Utc};
use serde::Serialize;

pub static OAUTH_STATE: &str = "oauth_state";
pub static SESSION: &str = "session";

#[derive(Serialize, Debug)]
pub struct Session {
    pub spotify_id: String,
    pub expires: DateTime<Utc>,
}
