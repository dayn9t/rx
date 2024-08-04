use crate::serde_export::{Deserialize, Serialize};
use crate::time::ClockTime;

/// 时钟时段
#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Default, Debug)]
pub struct ClockPeriod {
    pub start: ClockTime,
    pub end: ClockTime,
}

/// 时钟时段集合
pub type ClockPeriods = Vec<ClockPeriod>;

/// 工作表
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScheduleInfo {
    /// 按星期的排班信息
    pub days: [ClockPeriods; 7],
}

pub type ScheduleInfoVec = Vec<ScheduleInfo>;
