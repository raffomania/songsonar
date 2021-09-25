use crate::basics::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub static OAUTH_STATE: &str = "oauth_state";
pub static SESSION: &str = "session";

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub spotify_id: String,
    pub expires: DateTime<Utc>,
}

impl Session {
    pub fn from_str(input: &str) -> Result<Session> {
        let deserialized = serde_json::from_str(input)?;

        Ok(deserialized)
    }
}
