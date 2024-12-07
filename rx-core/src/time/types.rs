pub use chrono::ParseResult;
pub use chrono::prelude::*;
use core::range::Range;

/// UTC 时间
pub type UtcDateTime = DateTime<Utc>;
/// 本地时间
pub type LocalDateTime = DateTime<Local>;

/// 时间段
pub use time::Duration;

/// 时间范围
pub type TimeRange = Range<LocalDateTime>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_time_str() {
        println!("LocalDateTime size: {}", size_of::<LocalDateTime>());
        println!("NaiveDateTime size: {}", size_of::<NaiveDateTime>());
        println!("NaiveTime size: {}", size_of::<NaiveTime>());

        let time = Local::now().time();
        println!("NaiveTime now: {}", time);
    }
}
