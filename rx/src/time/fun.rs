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
pub fn to_local_str(dt: DateTime) -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    format!("{}", dt.format_with_items(fmt))
}

/// 获取当前时间本地字符串（不含有毫秒）
pub fn now() -> DateTime {
    Local::now().naive_local()
}

/// 获取当前时间本地字符串（不含有毫秒）
pub fn local_time_str() -> String {
    to_local_str(now())
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
