use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::OffsetComponents;

#[derive(Debug, Copy)]
pub enum TimeFormat {
    RFC3339,
    ISO8601,
    HumanReadable,
    Timestamp,
    Now,
}

pub fn parse_time(time_str: &str) -> Result<(String, DateTime<Utc>, TimeFormat)> {
    // Handle "now" keyword
    if time_str.to_lowercase() == "now" {
        return Ok((String::from("now"), Utc::now(), TimeFormat::Now));
    }

    // Try parsing as RFC3339 first (e.g., '2024-01-01T12:00:00Z')
    if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        return Ok((
            time_str.to_string(),
            dt.with_timezone(&Utc),
            TimeFormat::RFC3339,
        ));
    }

    // Try parsing as ISO8601 (e.g., '2024-01-01T12:00:00+00:00')
    if let Ok(dt) = DateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S%:z") {
        return Ok((
            time_str.to_string(),
            dt.with_timezone(&Utc),
            TimeFormat::ISO8601,
        ));
    }

    // Try parsing as Unix timestamp
    if let Ok(timestamp) = time_str.parse::<i64>() {
        match Utc.timestamp_opt(timestamp, 0) {
            chrono::LocalResult::Single(dt) => {
                return Ok((time_str.to_string(), dt, TimeFormat::Timestamp));
            }
            _ => return Err(anyhow!("Invalid timestamp")),
        }
    }

    // Try parsing as human-readable format (e.g., '2024-01-01 12:00:00')
    if let Ok(dt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        return Ok((
            time_str.to_string(),
            Utc.from_utc_datetime(&dt),
            TimeFormat::HumanReadable,
        ));
    }

    Err(anyhow!(
        "Invalid time format. Expected: timestamp, 'now', RFC3339 (e.g., '2024-01-01T12:00:00Z'), ISO8601 (e.g., '2024-01-01T12:00:00+00:00'), or date time string (e.g., '2024-01-01 12:00:00')"
    ))
}

pub fn format_timezone_output(
    _timezone: &str,
    converted: DateTime<chrono_tz::Tz>,
    offset: impl OffsetComponents,
    format: TimeFormat,
) -> String {
    let total_offset = offset.base_utc_offset() + offset.dst_offset();
    let hours = total_offset.num_hours();
    let minutes = total_offset.num_minutes() % 60;
    let offset_str = format!("{:+03}:{:02}", hours, minutes.abs());
    let tz_name = converted.format("%Z").to_string();

    let formatted_time = match format {
        TimeFormat::RFC3339 | TimeFormat::ISO8601 => converted.format("%Y-%m-%dT%H:%M:%S"),
        TimeFormat::HumanReadable | TimeFormat::Timestamp | TimeFormat::Now => {
            converted.format("%Y-%m-%d %H:%M:%S")
        }
    };

    if tz_name.is_empty() {
        format!("{} ({})", formatted_time, offset_str)
    } else if tz_name
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == '-')
    {
        format!("{} {}", formatted_time, tz_name)
    } else {
        format!("{} {} ({})", formatted_time, tz_name, offset_str)
    }
}
