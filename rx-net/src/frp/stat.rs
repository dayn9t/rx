use crate::frp::*;
use core::range::Range;
use regex::Regex;
use rx_core::log::{debug, info};
use rx_core::text::BoxResult;
use rx_core::time::NaiveDateTime;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct NodeStat {
    pub name: String,
    pub time_range: Range<NaiveDateTime>,
}

#[derive(Debug)]
pub struct TimeStat {
    items: HashMap<String, Range<NaiveDateTime>>,
    include: String,
    exclude: String,
}

impl TimeStat {
    pub fn new() -> Self {
        TimeStat {
            items: HashMap::new(),
            include: "name-".to_string(),
            exclude: "clos".to_string(),
        }
    }

    pub fn update(&mut self, key: String, time: NaiveDateTime) {
        let range = self.items.entry(key).or_insert(Range {
            start: time,
            end: time,
        });
        if time < range.start {
            range.start = time;
        }
        if time > range.end {
            range.end = time;
        }
    }

    /// 从日志文件更新统计信息
    pub fn update_with_log(&mut self, log_file: &Path) -> BoxResult<()> {
        let file = File::open(log_file)?;
        let reader = BufReader::new(file);

        // 匹配 ANSI 颜色码的正则表达式
        let re = Regex::new(r"\x1b\[[0-9;]*m")?;

        for line in reader.lines() {
            let line = line?;

            if line.find(&self.include).is_none() {
                continue;
            }
            if line.find(&self.exclude).is_some() {
                continue;
            }

            // 移除颜色信息
            let line = re.replace_all(&line, "").to_string();

            if let Ok(msg) = UserConnectionMsg::from_str(&line) {
                //println!("#{} {:?}", i, msg.time);
                self.update(msg.name, msg.time);
            } else if let Ok(msg) = ProxySuccessMsg::from_str(&line) {
                //println!("#{} {:?}", i, msg.time
                self.update(msg.name, msg.time);
            } else if let Ok(msg) = ProxyExistMsg::from_str(&line) {
                //println!("#{} {:?}", i, msg.time
                self.update(msg.name, msg.time);
            } else if let Ok(msg) = ProxyListenMsg::from_str(&line) {
                //println!("#{} {:?}", i, msg.time
                self.update(msg.name, msg.time);
            } else {
                debug!("Unrecognized log entry: {}", line);
            }
        }
        info!("nodes len: {}", self.items.len());
        Ok(())
    }

    pub fn get_new_nodes(&self, start_time: NaiveDateTime) -> Vec<NodeStat> {
        self.items
            .iter()
            .find(|(_, range)| range.start > start_time)
            .map(|(name, range)| NodeStat {
                name: name.clone(),
                time_range: range.clone(),
            })
            .into_iter()
            .collect()
    }
    pub fn get_all_nodes(&self) -> Vec<NodeStat> {
        self.items
            .iter()
            .map(|(name, range)| NodeStat {
                name: name.clone(),
                time_range: range.clone(),
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_update_with_log() {
        // 创建临时目录和日志文件
        let dir = tempdir().unwrap();
        let log_file_path = dir.path().join("test.log");
        let mut log_file = File::create(&log_file_path).unwrap();

        // 写入测试日志内容
        writeln!(log_file, "2024/08/15 15:15:53 [I] [proxy.go:199] [9f9d61c865a4249c] [name-51048] get a user connection [94.156.64.251:46078]").unwrap();
        writeln!(log_file, "2024/08/15 15:15:53 [W] [control.go:493] [c34d45ac838c9177] new proxy [name-51054] type [tcp] error: proxy [name-51054] already exists").unwrap();

        // 创建 TimeStat 实例并更新统计信息
        let mut time_stat = TimeStat::new();
        time_stat.update_with_log(&log_file_path).unwrap();

        // 验证解析结果
        let expected_time_1 = parse_time("2024/08/15 15:15:53").unwrap();
        let expected_time_2 = parse_time("2024/08/15 15:15:53").unwrap();
        assert_eq!(
            time_stat.items.get("name-51048").unwrap().start,
            expected_time_1
        );
        assert_eq!(
            time_stat.items.get("name-51054").unwrap().start,
            expected_time_2
        );
    }
}
