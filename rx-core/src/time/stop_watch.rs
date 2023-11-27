use std::fmt;

use super::fun::*;

/// 秒表
#[derive(Default, Debug)]
pub struct StopWatch {
    started: bool,
    elapsed: i64,
    start: i64,
    count: usize,
}

impl StopWatch {
    /// 创建新秒表
    pub fn new() -> Self {
        Self::default()
    }

    ///启动计时，总时间累计
    pub fn start(&mut self) {
        if !self.started {
            self.count += 1;
            self.start = nsecs_since_epoch();
            self.started = true;
        }
    }

    ///重新启动计时
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    ///停止计时
    pub fn stop(&mut self) -> f64 {
        if !self.has_started() {
            return 0.0;
        };
        let d = nsecs_since_epoch() - self.start;
        self.elapsed += d;
        self.started = false;
        d as f64 / 1_000_000_000.0
    }

    ///复位
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    ///是否已经启动
    pub fn has_started(&self) -> bool {
        self.started
    }

    ///流逝的总时间
    pub fn elapsed(&self) -> f64 {
        self.elapsed as f64 / 1_000_000_000.0
    }

    ///计数次数
    pub fn count(&self) -> usize {
        self.count
    }

    ///流逝的平均时间
    pub fn average(&self) -> f64 {
        self.elapsed() / self.count as f64
    }
}

impl fmt::Display for StopWatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "elapsed: {} count: {} average: {}",
            self.elapsed(),
            self.count(),
            self.average()
        )
    }
}
