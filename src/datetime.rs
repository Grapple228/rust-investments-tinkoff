use chrono::{NaiveDate, NaiveTime, Timelike};
use prost_types::Timestamp;

/// Wrapper for `prost_types::Timestamp`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTime {
    timestamp: Timestamp,
}

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

    /// Returns current timestamp
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use tokio::time::sleep;

    use super::*;

    #[test]
    fn test_datetime_from_ymd_ok() -> Result<()> {
        const YWD_SECONDS: i64 = 1640995200;
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
