use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

pub fn make_zulu_timestamp(utc_date_time: NaiveDateTime) -> String {
    let date_time_zulu: DateTime<Utc> = DateTime::from_utc(utc_date_time, Utc);
    date_time_zulu.format("%FT%H:%M:%SZ").to_string()
}
