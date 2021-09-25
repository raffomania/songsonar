use crate::basics::*;
use chrono::{DateTime, Utc};
use miniserde::{Deserialize, Serialize};

pub static OAUTH_STATE: &str = "oauth_state";
pub static SESSION: &str = "session";

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSession {
    pub spotify_id: String,
    pub expires: String,
}

#[derive(Debug)]
pub struct Session {
    pub spotify_id: String,
    pub expires: DateTime<Utc>,
}

impl Session {
    pub fn from_str(input: &str) -> Result<Session> {
        let deserialized: JsonSession = miniserde::json::from_str(input)?;
        let expires =
            chrono::DateTime::parse_from_rfc3339(&deserialized.expires)?
                .with_timezone(&Utc);

        Ok(Session {
            expires,
            spotify_id: deserialized.spotify_id,
        })
    }

    pub fn to_string(session: Session) -> String {
        let expires = session.expires.to_rfc3339();
        let json_session = JsonSession {
            spotify_id: session.spotify_id,
            expires,
        };

        miniserde::json::to_string(&json_session)
    }
}
