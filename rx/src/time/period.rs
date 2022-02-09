use core::ops::Range;
use std::cmp;

use serde_derive::Serialize;

use super::timestamp::*;
use super::types::*;

/// 时间段
#[derive(PartialEq, Copy, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl Period {
    /// 创建时间段
    pub fn new(start: Timestamp, end: Timestamp) -> Period {
        Period { start, end }
    }

    /// 创建时间段 - 根据可选时间
    pub fn option(start: &Option<Timestamp>, end: &Option<Timestamp>) -> Period {
        let start = start.unwrap_or(Timestamp::min_value());
        let end = end.unwrap_or(Timestamp::max_value());

        Self::new(start, end)
    }

    /// 创建时间段 - 根据时间
    pub fn from_datetime(start: NaiveDateTime, end: NaiveDateTime) -> Period {
        Period {
            start: Timestamp::from(start),
            end: Timestamp::from(end),
        }
    }

    /// 创建时间段 - 根据可选时间
    pub fn option_datetime(start: &Option<NaiveDateTime>, end: &Option<NaiveDateTime>) -> Period {
        let start = timestamp_or(&start, Timestamp::min_value());
        let end = timestamp_or(&end, Timestamp::max_value());

        Self::new(start, end)
    }

    /// 创建时间段 - 时间点
    pub fn point(time: Timestamp) -> Period {
        Period {
            start: time,
            end: time,
        }
    }

    /// 扩展时间段以便包含新时间
    pub fn expand(&mut self, time: Timestamp) {
        self.start = cmp::min(self.start, time);
        self.end = cmp::max(self.end, time);
    }

    /// 扩展时间段以便包含新时间
    pub fn expand_by(&mut self, p: &Period) {
        *self = self.or1(p);
    }

    /// 时间段长度
    pub fn len(&self) -> i64 {
        self.end - self.start
    }

    /// 时间段是否有效
    pub fn valid(&self) -> bool {
        self.end >= self.start
    }

    /// 时间段交集
    pub fn and1(&self, p: &Period) -> Period {
        Period {
            start: cmp::max(self.start, p.start),
            end: cmp::min(self.end, p.end),
        }
    }

    /// 时间段并集
    pub fn or1(&self, p: &Period) -> Period {
        Period {
            start: cmp::min(self.start, p.start),
            end: cmp::max(self.end, p.end),
        }
    }
}

impl Into<Period> for Range<Timestamp> {
    fn into(self) -> Period {
        Period {
            start: self.start,
            end: self.end,
        }
    }
}

impl Into<Range<Timestamp>> for Period {
    fn into(self) -> Range<Timestamp> {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

/*
let ts = Timestamp::new(1557506652);

let s = to_json(&ts).unwrap();
let ts1: Timestamp = serde_json::from_str(&s).unwrap();

println!("time: {}", to_json(&ts).unwrap());
println!("time1: {}", to_json(&ts1).unwrap());

let time = NaiveDateTime::from_timestamp(1557506652, 0);

println!("time: {}", to_json(&time).unwrap());
*/
