use chrono::{DateTime, Datelike, Duration, Utc, Weekday};

pub async fn schedule_updates() {
    let mut interval = friday_midnight_interval();
    loop {
        println!("{:?} before", interval);
        interval.tick().await;
        println!("{:?} after", interval);
    }
}

fn friday_midnight_interval() -> tokio::time::Interval {
    tokio::time::interval_at(
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
    let until_friday = Duration::days(days_until_friday.into());

    let tomorrow_midnight = (now + Duration::days(1)).date().and_hms(0, 0, 0);
    let until_tomorrow_midnight = tomorrow_midnight.signed_duration_since(now);

    return until_friday + until_tomorrow_midnight;
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
