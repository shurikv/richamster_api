use crate::models::common::TransactionType;
use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Deserializer};

pub fn timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: f64 = serde::Deserialize::deserialize(deserializer)?;
    let secs = ts.trunc() as i64;
    let nanos = ((ts.fract()) * 1_000_000_000.0).round() as u32;
    let dt = DateTime::from_timestamp(secs, nanos).unwrap();
    Ok(Local.from_utc_datetime(&dt.naive_utc()))
}

pub fn date_string_deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    let result = opt
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Local));
    Ok(result)
}

pub fn string_timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = serde::Deserialize::deserialize(deserializer)?;
    if timestamp.is_none() {
        return Err(serde::de::Error::custom("Timestamp is None"));
    }
    let date_time = DateTime::from_timestamp(timestamp.unwrap().parse().unwrap(), 0).unwrap();
    let local_datetime = Local.from_utc_datetime(&date_time.naive_utc());
    Ok(local_datetime)
}

pub fn option_timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = serde::Deserialize::deserialize(deserializer)?;
    if timestamp.is_none() {
        return Ok(None);
    }
    let date_time = DateTime::from_timestamp(timestamp.unwrap().parse().unwrap(), 0).unwrap();
    let local_datetime = Local.from_utc_datetime(&date_time.naive_utc());
    Ok(Some(local_datetime))
}

fn transaction_type_deserialize<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
where
    D: Deserializer<'de>,
{
    let transaction_type: i32 = serde::Deserialize::deserialize(deserializer)?;
    let tr_type: TransactionType = transaction_type.into();
    Ok(tr_type)
}
