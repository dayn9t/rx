use core::str::FromStr;
use std::fmt;

use serde::{Deserialize, Deserializer, Serializer};

use crate::serde_export::Serialize;

/// 时钟时间（时分秒）
#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct ClockTime {
    secs: u32,
}

impl ClockTime {
    /// 返回时钟时间的小时数
    pub fn hour(&self) -> u32 {
        self.secs / 3600
    }

    /// 返回时钟时间的分钟数
    pub fn minute(&self) -> u32 {
        (self.secs % 3600) / 60
    }

    /// 返回时钟时间的秒数
    pub fn second(&self) -> u32 {
        self.secs % 60
    }
}

impl fmt::Display for ClockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

impl FromStr for ClockTime {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err("expected a string in the format HH:MM:SS".into());
        }

        let hour: u32 = parts[0].parse()?;
        let minute: u32 = parts[1].parse()?;
        let second: u32 = parts[2].parse()?;

        let secs = hour * 3600 + minute * 60 + second;

        Ok(ClockTime { secs })
    }
}

impl Serialize for ClockTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for ClockTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ClockTime::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let clock_time = ClockTime::from_str("12:34:56").unwrap();
        assert_eq!(clock_time.hour(), 12);
        assert_eq!(clock_time.minute(), 34);
        assert_eq!(clock_time.second(), 56);
    }

    #[test]
    fn test_serialize() {
        let clock_time = ClockTime {
            secs: 12 * 3600 + 34 * 60 + 56,
        };
        let serialized = serde_json::to_string(&clock_time).unwrap();
        assert_eq!(serialized, "\"12:34:56\"");
    }

    #[test]
    fn test_deserialize() {
        let clock_time: ClockTime = serde_json::from_str("\"12:34:56\"").unwrap();
        assert_eq!(clock_time.hour(), 12);
        assert_eq!(clock_time.minute(), 34);
        assert_eq!(clock_time.second(), 56);
    }
}
