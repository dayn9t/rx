use anyhow::anyhow;
use chrono::Local;
use std::collections::HashMap;
use tokio::sync::Mutex;

use super::types::{COMPLETED, ERROR, IN_PROGRESS, NOT_STARTED, TaskInfo, TaskStatusInfo};
use crate::api::dao_list_client::{DaoListClient, ParamsMap, ResultE};

/// 任务客户端
///
/// 提供对任务及其状态的管理功能
pub struct TaskClient {
    /// 内部API客户端
    client: Mutex<DaoListClient>,
    /// 任务表名称
    task_table_name: String,
    /// 状态表名称
    status_table_name: String,
    /// 默认查询参数
    params: HashMap<String, String>,
}

impl TaskClient {
    /// 创建任务客户端
    ///
    /// # 参数
    /// * `base_url` - API的基础URL
    /// * `task_table_name` - 任务表名称
    /// * `status_table_name` - 状态表名称
    /// * `partition_id` - 分区ID
    pub fn new(
        base_url: impl Into<String>,
        task_table_name: Option<&str>,
        status_table_name: Option<&str>,
        partition_id: &str,
    ) -> Self {
        let mut params = HashMap::new();
        params.insert("partition_id".to_string(), partition_id.into());

        Self {
            client: Mutex::new(DaoListClient::new(base_url)),
            task_table_name: task_table_name.unwrap_or("task").to_string(),
            status_table_name: status_table_name.unwrap_or("status").to_string(),
            params,
        }
    }

    fn merge_params(&self, extra: Option<&HashMap<String, String>>) -> HashMap<String, String> {
        let mut final_params = self.params.clone();
        if let Some(p) = extra {
            for (k, v) in p {
                final_params.insert(k.clone(), v.clone());
            }
        }
        final_params
    }

    /// 设置认证令牌
    ///
    /// # 参数
    /// * `token` - 认证令牌字符串
    pub async fn set_auth_token(&self, token: impl Into<String>) {
        let client = self.client.lock().await;
        client.set_auth_token(token).await;
    }

    /// 获取全部任务列表，可选过滤参数
    ///
    /// # 参数
    /// * `params` - 查询参数字典，用于过滤结果集
    ///
    /// # 返回
    /// * `ResultE<Vec<TaskInfo>>` - 所有任务记录列表
    pub async fn get_all_tasks(&self, params: Option<&ParamsMap>) -> ResultE<Vec<TaskInfo>> {
        let params = self.merge_params(params);
        let client = self.client.lock().await;
        client.get_all(&self.task_table_name, Some(&params)).await
    }

    /// 添加新任务记录
    ///
    /// # 参数
    /// * `task` - 任务记录实例
    ///
    /// # 返回
    /// * `ResultE<TaskInfo>` - 添加后的任务记录
    pub async fn add_task(&self, task: TaskInfo) -> ResultE<TaskInfo> {
        let client = self.client.lock().await;
        client
            .post(&self.task_table_name, task, Some(&self.params))
            .await
    }

    /// 获取全部任务状态列表，可选过滤参数
    ///
    /// # 参数
    /// * `params` - 查询参数字典，用于过滤结果集
    ///
    /// # 返回
    /// * `ResultE<Vec<TaskStatusInfo>>` - 所有任务状态记录列表
    pub async fn get_all_statuses(
        &self,
        params: Option<&ParamsMap>,
    ) -> ResultE<Vec<TaskStatusInfo>> {
        let params = self.merge_params(params);
        let client = self.client.lock().await;
        client.get_all(&self.status_table_name, Some(&params)).await
    }

    /// 获取指定任务的状态信息
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 任务状态信息
    pub async fn get_status(&self, task_id: &str) -> ResultE<TaskStatusInfo> {
        let client = self.client.lock().await;
        client
            .get(&self.status_table_name, task_id, Some(&self.params))
            .await
    }

    /// 获取指定任务信息
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskInfo>` - 任务信息
    pub async fn get_task(&self, task_id: &str) -> ResultE<TaskInfo> {
        let client = self.client.lock().await;
        client
            .get(&self.task_table_name, task_id, Some(&self.params))
            .await
    }

    /// 找到可执行任务
    ///
    /// 查找状态为未启动且已启用的任务
    ///
    /// # 返回
    /// * `ResultE<(TaskInfo, TaskStatusInfo)>` - 任务信息和状态的元组
    pub async fn find_task(&self) -> ResultE<(TaskInfo, TaskStatusInfo)> {
        // 创建查询参数
        let mut params = self.params.clone();
        params.insert("status".to_string(), NOT_STARTED.to_string());
        params.insert("enabled".to_string(), "true".to_string());

        // 获取所有任务状态
        let client = self.client.lock().await;
        let statuses = client
            .get_all::<TaskStatusInfo>(&self.status_table_name, Some(&params))
            .await?;

        if statuses.is_empty() {
            return Err(anyhow!("没有找到可执行的任务"));
        }

        // 获取第一个未启动任务的信息
        let first_status = &statuses[0];
        let task_id = first_status
            .id
            .as_ref()
            .ok_or_else(|| anyhow!("任务状态缺少ID"))?;

        let task = client
            .get::<TaskInfo>(&self.task_table_name, task_id, Some(&self.params))
            .await
            .map_err(|e| anyhow!("获取任务信息失败: {}", e))?;

        Ok((task, first_status.clone()))
    }

    /// 开始执行指定任务
    ///
    /// 将任务状态设置为进行中，记录开始时间，设置进度为0
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    /// * `worker` - 可选的工作者标识
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn task_start(
        &self,
        task_id: &str,
        worker: Option<String>,
    ) -> ResultE<TaskStatusInfo> {
        // 获取当前状态
        let mut status = self.get_status(task_id).await?;

        if status.status != NOT_STARTED as i32 {
            return Err(anyhow!(
                "任务 #{} 当前状态为 {}，无法启动",
                task_id,
                status.status
            ));
        }

        // 更新状态为进行中
        status.status = IN_PROGRESS as i32;
        status.progress = 0;
        status.start_time = Some(Local::now());
        status.update_time = Some(Local::now());
        status.worker_id = worker;

        // 提交更新
        let client = self.client.lock().await;
        client
            .put(&self.status_table_name, status, Some(&self.params))
            .await
    }

    /// 终结指定任务
    ///
    /// 将任务状态设置为已完成，进度设为100%
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn task_done(&self, task_id: &str) -> ResultE<TaskStatusInfo> {
        self.update_progress(task_id, 100, Some(COMPLETED as i32))
            .await
    }

    /// 标记指定任务为出错
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn task_error(&self, task_id: &str) -> ResultE<TaskStatusInfo> {
        // 获取当前状态
        let mut status = self.get_status(task_id).await?;

        // 更新状态为出错
        status.status = ERROR as i32;
        status.update_time = Some(Local::now());

        // 提交更新
        let client = self.client.lock().await;
        client
            .put(&self.status_table_name, status, Some(&self.params))
            .await
    }

    /// 更新指定任务进度
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    /// * `progress` - 进度值(0-100)
    /// * `status` - 可选的状态更新
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn update_progress(
        &self,
        task_id: &str,
        progress: u32,
        status: Option<i32>,
    ) -> ResultE<TaskStatusInfo> {
        if progress > 100 {
            return Err(anyhow!("无效的进度值: {}，必须在 0-100 之间", progress));
        }

        // 获取当前状态
        let mut status_info = self.get_status(task_id).await?;

        // 更新进度
        status_info.progress = progress;
        status_info.update_time = Some(Local::now());

        // 如果提供了新状态，则更新
        if let Some(s) = status {
            status_info.status = s;
        // 如果进度为100%，自动设置状态为已完成
        } else if progress == 100 && status_info.status != ERROR as i32 {
            status_info.status = COMPLETED as i32;
        }

        // 提交更新
        let client = self.client.lock().await;
        client
            .put(&self.status_table_name, status_info, Some(&self.params))
            .await
    }

    /// 更新任务状态记录
    ///
    /// # 参数
    /// * `status` - 任务状态记录
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn update_task_status(&self, status: TaskStatusInfo) -> ResultE<TaskStatusInfo> {
        let client = self.client.lock().await;
        client
            .put(&self.status_table_name, status, Some(&self.params))
            .await
    }

    /// 启用任务
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn enable_task(&self, task_id: &str) -> ResultE<TaskStatusInfo> {
        self.enable_disable_task(task_id, true).await
    }

    /// 禁用任务
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    pub async fn disable_task(&self, task_id: &str) -> ResultE<TaskStatusInfo> {
        self.enable_disable_task(task_id, false).await
    }

    /// 启用或禁用任务
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    /// * `enable` - 是否启用
    ///
    /// # 返回
    /// * `ResultE<TaskStatusInfo>` - 更新后的任务状态
    async fn enable_disable_task(&self, task_id: &str, enable: bool) -> ResultE<TaskStatusInfo> {
        // 获取当前状态
        let mut status = self.get_status(task_id).await?;

        // 更新启用状态
        status.enabled = enable;
        status.update_time = Some(Local::now());

        // 更新状态
        self.update_task_status(status).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
