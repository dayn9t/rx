pub use chrono::ParseResult;
pub use chrono::prelude::*;
use core::range::Range;

/// UTC 时间
pub type UtcDateTime = DateTime<Utc>;
/// 本地时间
pub type LocalDateTime = DateTime<Local>;

/// 时间段
pub use std::time::Duration;

/// 时间范围
pub type TimeRange = Range<LocalDateTime>;

/// 将本地时间转换为UTC时间
pub fn to_utc(datetime: LocalDateTime) -> UtcDateTime {
    datetime.with_timezone(&Utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::json;

    #[test]
    fn test_local_time_str() {
        println!("LocalDateTime size: {}", size_of::<LocalDateTime>());
        println!("NaiveDateTime size: {}", size_of::<NaiveDateTime>());
        println!("NaiveTime size: {}", size_of::<NaiveTime>());

        let datetime = Local::now();
        println!("LocalDateTime now: {}", datetime);
        println!(
            "LocalDateTime now json: {}",
            json::to_pretty(&datetime).unwrap()
        );
        println!("LocalDateTime now (local): {}", datetime.to_rfc2822());

        let utc_datetime = to_utc(datetime);
        println!("UtcDateTime now: {}", utc_datetime);
        println!(
            "UtcDateTime now json: {}",
            json::to_pretty(&utc_datetime).unwrap()
        );

        let time = Local::now().time();
        println!("NaiveTime now: {}", time);
    }
}
