use std::ops::Sub;

use chrono::format::strftime::StrftimeItems;
use chrono::prelude::Local;
pub use chrono::{NaiveDateTime as DateTime, ParseResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 时间戳
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub struct Timestamp(u32);

impl Timestamp {
    /// 创建时间戳
    pub fn new(secs: u32) -> Self {
        Timestamp(secs)
    }

    /// 从字符串解析
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<Timestamp> {
        let dt = DateTime::parse_from_str(s, fmt)?;
        Ok(Timestamp::from(dt))
    }

    /// 从字符串解析
    pub fn parse_from_common_str(s: &str) -> ParseResult<Timestamp> {
        Self::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f")
    }

    /// 获取最小值
    pub fn min_value() -> Self {
        Timestamp(u32::min_value())
    }

    /// 获取最大值
    pub fn max_value() -> Self {
        Timestamp(u32::max_value())
    }

    /// 获取时间戳的所有秒数
    pub fn seconds(&self) -> u32 {
        self.0
    }

    /// 饱和减法
    pub fn sub_sat(&self, v: u32) -> Self {
        Self(if self.0 > v { self.0 - v } else { 0 })
    }
}

impl Sub for Timestamp {
    type Output = i64;

    fn sub(self, other: Self) -> Self::Output {
        self.0 as i64 - other.0 as i64
    }
}

impl From<DateTime> for Timestamp {
    fn from(dt: DateTime) -> Self {
        Timestamp(dt.timestamp() as u32)
    }
}

impl Into<DateTime> for Timestamp {
    fn into(self) -> DateTime {
        DateTime::from_timestamp(self.0 as i64, 0)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt: DateTime = self.clone().into();
        dt.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dt = DateTime::deserialize(deserializer);
        dt.map(|v| Timestamp::from(v))
    }
}

/// 日期时间转换成时间戳
pub fn timestamp_or(time: &Option<DateTime>, v: Timestamp) -> Timestamp {
    if let Some(t) = time {
        Timestamp::from(t.to_owned())
    } else {
        v
    }
}

/*
let ts = Timestamp::new(1557506652);

let s = to_json(&ts).unwrap();
let ts1: Timestamp = serde_json::from_str(&s).unwrap();

println!("time: {}", to_json(&ts).unwrap());
println!("time1: {}", to_json(&ts1).unwrap());

let time = DateTime::from_timestamp(1557506652, 0);

println!("time: {}", to_json(&time).unwrap());
*/

/// 本地字符串（不含有毫秒）
pub fn local_time_str() -> String {
    let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S").clone();
    let dt = Local::now().naive_local();
    format!("{}", dt.format_with_items(fmt))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_local_time_str() {
        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
    }
}
