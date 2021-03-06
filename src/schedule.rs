use aspotify::Client;
use chrono::{DateTime, Datelike, Duration, Utc, Weekday};
use sentry::integrations::anyhow::capture_anyhow;
use sqlx::{Pool, Postgres};

use crate::basics::*;

use crate::db::Transaction;
use crate::{db::create_db_pool, spotify, storage};

pub async fn schedule_updates() -> Result<()> {
    let (delay, period) = friday_midnight_interval();
    let delay_seconds = delay - tokio::time::Instant::now();
    let next_run = Utc::now() + Duration::from_std(delay_seconds).unwrap();
    log::info!("Next run scheduled at {}", next_run.to_rfc2822());

    let mut interval = tokio::time::interval_at(delay, period);

    let client = spotify::get_client().unwrap();
    let pool = create_db_pool().await.unwrap();

    loop {
        interval.tick().await;

        update_all_playlists(&pool, &client)
            .await
            .as_ref()
            .map_err(capture_anyhow)
            .ok();
    }
}

async fn update_all_playlists(
    pool: &Pool<Postgres>,
    client: &Client,
) -> Result<()> {
    let mut tx = Transaction(pool.begin().await?);
    let users = storage::users::list_users(&mut tx).await?;
    tx.0.commit().await?;

    log::info!("Found {} users to update", users.len());

    for user in users {
        log::debug!("updating user {:?}", user);

        client.set_refresh_token(Some(user.refresh_token)).await;
        client
            .set_current_access_token(
                user.access_token,
                std::time::Instant::now(),
            )
            .await;

        let playlist_id = if let Some(id) = user.playlist_id {
            id
        } else {
            capture_anyhow(&anyhow!("Missing playlist for user!"));
            continue;
        };

        spotify::update_playlist(
            client,
            user.weeks_in_playlist.unwrap_or(1),
            &playlist_id,
        )
        .await
        .as_ref()
        .map_err(capture_anyhow)
        .ok();
    }

    Ok(())
}

fn friday_midnight_interval() -> (tokio::time::Instant, std::time::Duration) {
    (
        tokio::time::Instant::now()
            + next_friday_midnight(Utc::now())
                .to_std()
                .expect("Illegal negative duration"),
        Duration::days(7)
            .to_std()
            .expect("Illegal negative duration"),
    )
}

fn next_friday_midnight(now: DateTime<Utc>) -> Duration {
    let weekday_number: i64 = now.weekday().num_days_from_monday().into();
    let friday_number: i64 = Weekday::Fri.num_days_from_monday().into();
    let mut days_until_friday = friday_number - weekday_number - 1;
    if days_until_friday < 0 {
        days_until_friday += 7;
    }
    let until_friday = Duration::days(days_until_friday);

    let tomorrow_midnight = (now + Duration::days(1)).date().and_hms(0, 0, 0);
    let until_tomorrow_midnight = tomorrow_midnight.signed_duration_since(now);

    until_friday + until_tomorrow_midnight
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn next_friday_midnight_is_correct() {
        // saturday midnight to friday midnight
        let now = Utc.ymd(2021, 09, 25).and_hms(0, 0, 0);
        let result = next_friday_midnight(now);
        assert_eq!(result.num_days(), 6);
        assert_eq!(result.num_hours() % 24, 0);
        assert_eq!(result.num_minutes() % 60, 0);

        // thursday noon to friday midnight
        let now = Utc.ymd(2021, 09, 30).and_hms(12, 30, 0);
        let result = next_friday_midnight(now);
        assert_eq!(result.num_days(), 0);
        assert_eq!(result.num_hours() % 24, 11);
        assert_eq!(result.num_minutes() % 60, 30);
    }
}
