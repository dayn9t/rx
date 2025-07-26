use chrono::Local;
use poem_openapi::Object;
use rx_core::prelude::*;
use rx_core::time::LocalDateTime;
use rx_db::IRecord;
use rx_db::Record;

/// 未启动
pub const NOT_STARTED: u32 = 0;
/// 进行中
pub const IN_PROGRESS: u32 = 1;
/// 完成
pub const COMPLETED: u32 = 2;
/// 出错
pub const ERROR: u32 = 3;

/// 任务信息
///
/// 任务信息用于描述一个任务的基本信息.
#[derive(Object, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Record)]
#[serde(default)]
pub struct TaskInfo {
    /// 任务ID, 一般创建时为空, 由系统分配
    pub id: Option<usize>,
    /// 任务名称, 用于跟踪任务的进展
    pub name: Option<String>,
    /// 任务类别
    pub r#type: u32,
    /// 任务创建时间, 由系统自动填充
    pub created_at: Option<LocalDateTime>,
    /// 任务描述
    pub desc: Option<String>,
    /// 任务数据
    pub data: String,
}

impl TaskInfo {
    /// 默认任务名称
    pub fn default_name(&self) -> String {
        format!("任务-{}", self.unwrap_id())
    }

    /// 补全任务名称
    pub fn complete_name(&mut self) {
        if self.name.is_none() {
            self.name = Some(self.default_name());
        }
    }

    /// 补全任务
    pub fn complete(&self) -> Self {
        let mut task = self.clone();
        task.created_at = Some(Local::now());
        task.complete_name();
        task
    }
}

/// 任务状态信息
///
/// 描述一个任务的状态, 进展.
#[derive(Object, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Record)]
#[serde(default)]
pub struct TaskStatusInfo {
    /// 任务ID
    pub id: Option<usize>,
    /// 任务状态, 取值: 0-未开始, 1-进行中, 2-完成, 3-失败
    pub status: i32,
    /// 完成进度, 范围: 0-100
    pub progress: u32,
    /// 开始时间
    pub start_time: Option<LocalDateTime>,
    /// 结束时间
    pub update_time: Option<LocalDateTime>,
    /// 允许状态
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// 执行任务的工作者标识，例如线程名或进程名
    pub worker_id: Option<String>,
}

impl Default for TaskStatusInfo {
    fn default() -> Self {
        Self {
            id: None,
            status: 0,
            progress: 0,
            start_time: None,
            update_time: None,
            enabled: true,
            worker_id: None,
        }
    }
}

// 保留这个函数用于 serde 默认值
fn default_enabled() -> bool {
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_task() {}
}
