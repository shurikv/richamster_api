use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Deserializer};

pub fn timestamp_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: f64 = serde::Deserialize::deserialize(deserializer)?;
    let secs = ts.trunc() as i64;
    let nanos = ((ts.fract()) * 1_000_000_000.0).round() as u32;
    let dt = DateTime::from_timestamp(secs, nanos)
        .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
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
    let ts_str = timestamp.ok_or_else(|| serde::de::Error::custom("Timestamp is None"))?;
    let sec: i64 = ts_str.parse().map_err(serde::de::Error::custom)?;
    let date_time = DateTime::from_timestamp(sec, 0)
        .ok_or_else(|| serde::de::Error::custom("Timestamp out of range"))?;
    Ok(Local.from_utc_datetime(&date_time.naive_utc()))
}

pub fn option_timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: Option<String> = serde::Deserialize::deserialize(deserializer)?;
    let Some(ts_str) = timestamp else {
        return Ok(None);
    };
    let secs: i64 = ts_str.parse().map_err(serde::de::Error::custom)?;
    let date_time = DateTime::from_timestamp(secs, 0)
        .ok_or_else(|| serde::de::Error::custom("Timestamp out of range"))?;
    Ok(Some(Local.from_utc_datetime(&date_time.naive_utc())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::Deserialize;

    #[derive(Deserialize)]
    struct TimestampTest {
        #[serde(deserialize_with = "timestamp_deserialize")]
        ts: DateTime<Local>,
    }

    #[derive(Deserialize)]
    struct DateStringTest {
        #[serde(deserialize_with = "date_string_deserialize")]
        ts: Option<DateTime<Local>>,
    }

    #[derive(Deserialize)]
    struct StringTimestampTest {
        #[serde(deserialize_with = "string_timestamp_deserialize")]
        ts: DateTime<Local>,
    }

    #[derive(Deserialize)]
    struct OptionTimestampTest {
        #[serde(deserialize_with = "option_timestamp_deserialize")]
        ts: Option<DateTime<Local>>,
    }

    #[test]
    fn timestamp_deserialize_valid() {
        let json = r#"{"ts": 1700000000.0}"#;
        let result: TimestampTest = serde_json::from_str(json).unwrap();
        assert_eq!(result.ts.timestamp(), 1700000000);
    }

    #[test]
    fn timestamp_deserialize_with_fractional() {
        let json = r#"{"ts": 1700000000.5}"#;
        let result: TimestampTest = serde_json::from_str(json).unwrap();
        assert_eq!(result.ts.timestamp(), 1700000000);
        assert_eq!(result.ts.timestamp_subsec_nanos(), 500_000_000);
    }

    #[test]
    fn timestamp_deserialize_invalid_type() {
        let json = r#"{"ts": "not_a_number"}"#;
        let result = serde_json::from_str::<TimestampTest>(json);
        assert!(result.is_err());
    }

    #[test]
    fn date_string_deserialize_valid() {
        let json = r#"{"ts": "2023-11-14T22:13:20+00:00"}"#;
        let result: DateStringTest = serde_json::from_str(json).unwrap();
        assert!(result.ts.is_some());
        assert_eq!(result.ts.unwrap().timestamp(), 1700000000);
    }

    #[test]
    fn date_string_deserialize_null() {
        let json = r#"{"ts": null}"#;
        let result: DateStringTest = serde_json::from_str(json).unwrap();
        assert!(result.ts.is_none());
    }

    #[test]
    fn date_string_deserialize_invalid_format() {
        let json = r#"{"ts": "not-a-date"}"#;
        let result: DateStringTest = serde_json::from_str(json).unwrap();
        assert!(result.ts.is_none());
    }

    #[test]
    fn string_timestamp_deserialize_valid() {
        let json = r#"{"ts": "1700000000"}"#;
        let result: StringTimestampTest = serde_json::from_str(json).unwrap();
        assert_eq!(result.ts.timestamp(), 1700000000);
    }

    #[test]
    fn string_timestamp_deserialize_null() {
        let json = r#"{"ts": null}"#;
        let result = serde_json::from_str::<StringTimestampTest>(json);
        assert!(result.is_err());
    }

    #[test]
    fn string_timestamp_deserialize_not_a_number() {
        let json = r#"{"ts": "abc"}"#;
        let result = serde_json::from_str::<StringTimestampTest>(json);
        assert!(result.is_err());
    }

    // option_timestamp_deserialize tests

    #[test]
    fn option_timestamp_deserialize_valid() {
        let json = r#"{"ts": "1700000000"}"#;
        let result: OptionTimestampTest = serde_json::from_str(json).unwrap();
        assert!(result.ts.is_some());
        assert_eq!(result.ts.unwrap().timestamp(), 1700000000);
    }

    #[test]
    fn option_timestamp_deserialize_null() {
        let json = r#"{"ts": null}"#;
        let result: OptionTimestampTest = serde_json::from_str(json).unwrap();
        assert!(result.ts.is_none());
    }

    #[test]
    fn option_timestamp_deserialize_not_a_number() {
        let json = r#"{"ts": "abc"}"#;
        let result = serde_json::from_str::<OptionTimestampTest>(json);
        assert!(result.is_err());
    }
}
