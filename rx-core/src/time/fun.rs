use super::types::*;
use crate::text::BoxResult;
use chrono::format::strftime::StrftimeItems;

/// 获取纪元到现在秒数
pub fn secs_since_epoch() -> f64 {
    now().and_utc().timestamp_nanos_opt().unwrap() as f64 / 1000_000_000.0
}

/// 获取纪元到现在纳秒数
pub fn nsecs_since_epoch() -> i64 {
    now().and_utc().timestamp_nanos_opt().unwrap()
}

/// 时间转换本地字符串（不含有毫秒）
pub fn to_local_str(dt: NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    format!("{}", dt.format_with_items(fmt))
}

/// 时间转换本地时间字符串ID
pub fn to_local_id(dt: NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d_%H-%M-%S%.3f");
    format!("{}", dt.format_with_items(fmt))
}

/// 时间转换为ISO基本格式
pub fn to_iso_basic(time: &NaiveDateTime) -> String {
    format!("{}", time.format("%Y-%m-%dT%H:%M:%S"))
}

/// 时间转换本地时间字符串ID
pub fn to_local_iso_str(dt: NaiveDateTime) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%dT%H:%M:%S%.3f");
    format!("{}+08:00", dt.format_with_items(fmt))
}

/// 时间转换本地时间文件路径，如：2020-01-01/00-00-00.000.jpg
pub fn to_local_path(dt: NaiveDateTime, ext: &str) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d/%H-%M-%S%.3f");
    format!("{}{}", dt.format_with_items(fmt), ext)
}

/// 获取当前时间本地时间
pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

/// 当前时间转换为标准字符串
pub fn now_local_iso_str() -> String {
    to_local_iso_str(now())
}

/// 获取当前时间本地字符串（不含有毫秒）
pub fn local_time_str() -> String {
    to_local_str(now())
}

/// 获取当前时间本地字符串
pub fn local_time_id() -> String {
    to_local_id(now())
}

/// 获取当前时间本地字符串
pub fn local_time_path(ext: &str) -> String {
    to_local_path(now(), ext)
}

/// UTC时间转本地Naive时间
pub fn naive_local(dt: UtcDateTime) -> NaiveDateTime {
    dt.with_timezone(&Local).naive_local()
}

/// Naive时间转本地时间
pub fn naive_to_local(naive: NaiveDateTime) -> LocalDateTime {
    Local.from_local_datetime(&naive).unwrap()
}

/// 今天起点的时间
pub fn today_midnight() -> NaiveDateTime {
    let today: NaiveDate = Local::now().date_naive();
    NaiveDateTime::new(today, NaiveTime::default())
}

/// 从字符串(无时区)解析本地时间
pub fn parse_datetime_from_naive(datetime_str: &str) -> ParseResult<LocalDateTime> {
    let naive = if let Ok(d) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S") {
        Ok(d)
    } else {
        NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
    };
    naive.map(|d| Local.from_local_datetime(&d).unwrap())
}

/// 本地时间舍入到秒
pub fn round_to_seconds(datetime: LocalDateTime) -> LocalDateTime {
    let nanos = datetime.timestamp_nanos_opt().unwrap() + 500_000_000;
    let secs = nanos / 1_000_000_000;
    Local.timestamp_opt(secs, 0).unwrap()
}

/// 解析RFC3339时间字符串并转为 NaiveDateTime
pub fn parse_rfc3339_to_naive(date_str: &str) -> BoxResult<NaiveDateTime> {
    let datetime_with_tz: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(date_str)?;
    Ok(datetime_with_tz.naive_local())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_time_str() {
        let s = now_local_iso_str();
        println!("{}", s);
        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
    }

    #[test]
    fn test_1() {
        let t1 = now();
        let t2 = now();
        let _d = t2 - t1;

        let _s1 = local_time_str();

        let _s1 = to_local_iso_str(t1);
        let _s1 = to_local_iso_str(t1);
        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
        let d1 = t1.date();
        println!("{}", d1.to_string())
    }

    #[test]
    fn test_parse_datetime_from_naive_valid() {
        let datetime_strs = ["2024-07-29T12:00:00", "2024-07-29 12:00:00"];
        for datetime_str in datetime_strs.iter() {
            let result = parse_datetime_from_naive(datetime_str);
            assert!(result.is_ok());
            let local_dt = result.unwrap();
            assert_eq!(local_dt.year(), 2024);
            assert_eq!(local_dt.month(), 7);
            assert_eq!(local_dt.day(), 29);
            assert_eq!(local_dt.hour(), 12);
            assert_eq!(local_dt.minute(), 0);
            assert_eq!(local_dt.second(), 0);
        }
    }

    #[test]
    fn test_parse_datetime_from_naive_invalid() {
        let datetime_str = "invalid-datetime";
        let result = parse_datetime_from_naive(datetime_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_rfc3339_to_naive_valid() {
        let date_str = "2024-07-29T12:00:00+00:00";
        let result = parse_rfc3339_to_naive(date_str).unwrap();
        assert_eq!(result.year(), 2024);
        assert_eq!(result.month(), 7);
        assert_eq!(result.day(), 29);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
    }
}
