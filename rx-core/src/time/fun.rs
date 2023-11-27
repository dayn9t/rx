use chrono::format::strftime::StrftimeItems;

use super::types::*;

/// 获取纪元到现在秒数
pub fn secs_since_epoch() -> f64 {
    now().timestamp_nanos() as f64 / 1000_000_000.0
}

/// 获取纪元到现在纳秒数
pub fn nsecs_since_epoch() -> i64 {
    now().timestamp_nanos()
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

/// 时间转换本地时间文件路径，如：2020-01-01/00-00-00.000.jpg
pub fn to_local_path(dt: NaiveDateTime, ext: &str) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d/%H-%M-%S%.3f");
    format!("{}{}", dt.format_with_items(fmt), ext)
}

/// 获取当前时间本地字符串（不含有毫秒）
pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_time_str() {
        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
    }

    #[test]
    fn test_1() {
        let t1 = now();
        let t2 = now();
        let _d = t2 - t1;

        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
    }
}
