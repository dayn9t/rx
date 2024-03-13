use std::fmt;
use std::ops::Sub;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::types::*;

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
        let dt = NaiveDateTime::parse_from_str(s, fmt)?;
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

    /// 转换成: NaiveDateTime
    pub fn to_datetime(&self) -> NaiveDateTime {
        self.to_owned().into()
    }
}

impl Sub for Timestamp {
    type Output = i64;

    fn sub(self, other: Self) -> Self::Output {
        self.0 as i64 - other.0 as i64
    }
}

impl From<NaiveDateTime> for Timestamp {
    fn from(dt: NaiveDateTime) -> Self {
        Timestamp(dt.and_utc().timestamp() as u32)
    }
}

impl Into<NaiveDateTime> for Timestamp {
    fn into(self) -> NaiveDateTime {
        DateTime::from_timestamp(self.0 as i64, 0)
            .unwrap()
            .naive_local()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_datetime().to_string())
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt: NaiveDateTime = self.clone().into();
        dt.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dt = NaiveDateTime::deserialize(deserializer);
        dt.map(|v| Timestamp::from(v))
    }
}

/// 日期时间转换成时间戳
pub fn timestamp_or(time: &Option<NaiveDateTime>, v: Timestamp) -> Timestamp {
    if let Some(t) = time {
        Timestamp::from(t.to_owned())
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let s = "2000-01-01 00:00:00";
        let t = Timestamp::parse_from_common_str(s).unwrap();
        assert_eq!(t.to_string(), s);
        println!("{}", t);

        let t1: NaiveDateTime = t.into();
        println!("{}", t1);
    }
}

/*
let ts = Timestamp::new(1557506652);

let s = json::to_pretty(&ts).unwrap();
let ts1: Timestamp = serde_json::from_str(&s).unwrap();

println!("time: {}", json::to_pretty(&ts).unwrap());
println!("time1: {}", json::to_pretty(&ts1).unwrap());

let time = NaiveDateTime::from_timestamp(1557506652, 0);

println!("time: {}", json::to_pretty(&time).unwrap());
*/
