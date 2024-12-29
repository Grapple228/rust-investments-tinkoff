mod error;

use chrono::{NaiveDate, NaiveTime, Timelike, Utc};
pub use error::{Error, Result};
use prost_types::Timestamp;

/// Wrapper for `prost_types::Timestamp`
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateTime {
    timestamp: Timestamp,
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.timestamp();
        let right = other.timestamp();

        if left.seconds == right.seconds {
            left.nanos.partial_cmp(&right.nanos)
        } else {
            left.seconds.partial_cmp(&right.seconds)
        }
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.timestamp();
        let right = other.timestamp();

        if left.seconds == right.seconds {
            left.nanos.cmp(&right.nanos)
        } else {
            left.seconds.cmp(&right.seconds)
        }
    }
}

impl core::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(date) = self.to_utc_string() {
            write!(f, "{}", date)
        } else {
            write!(f, "{}", self.timestamp)
        }
    }
}

// Constructors
impl DateTime {
    /// Creates a new `DateTime` for current time
    pub fn now() -> DateTime {
        let now = chrono::Utc::now();

        let timestamp = Timestamp {
            seconds: now.timestamp(),
            nanos: now.nanosecond() as i32,
        };

        DateTime { timestamp }
    }

    /// Creates a new `DateTime` from year, month and day
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<DateTime> {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            let datetime = date.and_time(NaiveTime::default());

            Some(DateTime {
                timestamp: Timestamp {
                    seconds: datetime.and_utc().timestamp(),
                    nanos: datetime.nanosecond() as i32,
                },
            })
        } else {
            None
        }
    }

    /// Creates a new `DateTime` from `prost_types::Timestamp`
    pub fn from_timestamp(timestamp: Timestamp) -> DateTime {
        DateTime { timestamp }
    }
}

impl DateTime {
    /// Returns current timestamp
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Add hours, minutes and seconds to current timestamp
    pub fn add_hms(&self, hours: i64, minutes: i64, seconds: i64) -> DateTime {
        let mut timestamp = self.timestamp;
        timestamp.seconds += hours * 3600 + minutes * 60 + seconds;

        DateTime { timestamp }
    }

    /// Add year, month and day to current timestamp
    pub fn add_ymd(&self, year: i32, month: u32, day: u32) -> DateTime {
        let mut timestamp = self.timestamp;

        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            timestamp.seconds += date.and_time(NaiveTime::default()).and_utc().timestamp();
        }

        DateTime { timestamp }
    }

    /// Add minutes to current timestamp
    pub fn add_minutes(&self, minutes: i64) -> DateTime {
        let mut timestamp = self.timestamp;
        timestamp.seconds += minutes * 60;

        DateTime { timestamp }
    }

    /// Add seconds to current timestamp
    pub fn add_seconds(&self, seconds: i64) -> DateTime {
        let mut timestamp = self.timestamp;
        timestamp.seconds += seconds;

        DateTime { timestamp }
    }

    /// Add hours to current timestamp
    pub fn add_hours(&self, hours: i64) -> DateTime {
        let mut timestamp = self.timestamp;
        timestamp.seconds += hours * 3600;

        DateTime { timestamp }
    }

    /// Add days to current timestamp
    pub fn add_days(&self, days: i64) -> DateTime {
        let mut timestamp = self.timestamp;
        timestamp.seconds += days * 86400;

        DateTime { timestamp }
    }
}

// To String conversions
impl DateTime {
    /// Converts `DateTime` to RFC3339 string
    pub fn to_utc_string(&self) -> Result<String> {
        let timestamp = self.timestamp();

        if let Some(utc_datetime) =
            chrono::DateTime::<Utc>::from_timestamp(timestamp.seconds, timestamp.nanos as u32)
        {
            Ok(utc_datetime.to_rfc3339())
        } else {
            Err(Error::TimestampConversionError)
        }
    }

    /// Converts `DateTime` to date string (YYYY-MM-DD)
    pub fn to_utc_date_string(&self) -> Result<String> {
        let timestamp = self.timestamp();

        if let Some(utc_datetime) =
            chrono::DateTime::<Utc>::from_timestamp(timestamp.seconds, timestamp.nanos as u32)
        {
            Ok(utc_datetime.date_naive().to_string())
        } else {
            Err(Error::TimestampConversionError)
        }
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    /// Unix timestamp for 2022-01-01
    const YWD_SECONDS: i64 = 1640995200;

    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use tokio::time::sleep;

    use super::*;

    #[test]
    fn test_to_string_ok() -> Result<()> {
        let fx_result = String::from("2022-01-01T05:30:34+00:00");

        let datetime = DateTime::from_timestamp(Timestamp {
            seconds: YWD_SECONDS,
            nanos: 0,
        })
        .add_hms(5, 30, 34);

        assert_eq!(datetime.to_utc_string()?, fx_result);

        Ok(())
    }

    #[test]
    fn test_to_date_string_ok() -> Result<()> {
        let fx_result = String::from("2022-01-01");

        let datetime = DateTime::from_timestamp(Timestamp {
            seconds: YWD_SECONDS,
            nanos: 0,
        })
        .add_hours(5)
        .add_minutes(30);

        assert_eq!(datetime.to_utc_date_string()?, fx_result);

        Ok(())
    }

    #[test]
    fn test_datetime_from_ymd_ok() -> Result<()> {
        let fx_date = DateTime::from_ymd(2022, 1, 1).expect("Failed to create datetime");

        assert_eq!(fx_date.timestamp().seconds, YWD_SECONDS);

        assert_eq!(fx_date.add_days(1).timestamp().seconds, YWD_SECONDS + 86400);

        Ok(())
    }

    #[tokio::test]
    async fn test_datetime_now_ok() -> Result<()> {
        let now = DateTime::now();

        // Sleep for 3 seconds
        sleep(std::time::Duration::from_secs(3)).await;

        let now_plus_3 = DateTime::now();

        // Check that now +3 seconds is greater than now
        assert!(now_plus_3.timestamp().seconds - now.timestamp().seconds >= 3);

        Ok(())
    }
}

// endregion: --- Tests
