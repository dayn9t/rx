use crate::api::{CodeResponse, DaoList};
use crate::task::{TaskInfo, TaskStatusInfo};
use poem::Result;
use poem_openapi::OpenApi;
/// API路径参数
pub use poem_openapi::param::Path; // FIXME: poem bug, 不能使用类型别名
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use rx_core::log::*;
use rx_db::IRecord;
use std::path::Path as FsPath;

/// API服务
pub struct TaskApiService {
    pub tasks: DaoList<TaskInfo>,
    pub status: DaoList<TaskStatusInfo>,
    //pub mqtt_cfg: DaoItem<MqttCfg>,
}

#[OpenApi]
impl TaskApiService {
    /// 打开API服务
    pub fn new(db_path: &FsPath, task_table_name: &str, status_table_name: &str) -> Self {
        info!("Loading data from {:?}", db_path);
        let tasks = DaoList::open_name(db_path, task_table_name).unwrap();
        let status = DaoList::open_name(db_path, status_table_name).unwrap();

        //let mqtt_cfg = Self::make_cfg(&app_params, "mqtt").unwrap();

        Self { tasks, status }
    }

    /// 获取全部任务
    #[oai(path = "/tasks", method = "get")]
    pub async fn task_get_all(
        &self,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<Vec<TaskInfo>>> {
        self.tasks.get_all(&Some(partition_id.0)).await
    }

    /// 获取指定任务
    #[oai(path = "/tasks/:id", method = "get")]
    pub async fn task_get(
        &self,
        id: Path<String>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<TaskInfo>> {
        self.tasks.get(&id, &Some(partition_id.0)).await
    }

    /// 添加一个任务
    #[oai(path = "/tasks", method = "post")]
    pub async fn task_post(
        &self,
        record: Json<TaskInfo>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<TaskInfo>> {
        let partition_id = Some(partition_id.0);

        let resp = self.tasks.post(record, &partition_id).await?;
        if let CodeResponse::Created(task) = resp {
            let task_id = task.unwrap_id();
            let task = task.0.complete();
            info!("Task created: {:?}", task);

            let status = TaskStatusInfo {
                id: Some(task_id.clone()),
                ..TaskStatusInfo::default()
            };
            self.status
                .put_record(&task_id, status, &partition_id)
                .await?;
            self.tasks
                .put(&Path(task_id), Json(task), &partition_id)
                .await
        } else {
            Ok(resp)
        }
    }

    /// 删除指定任务
    #[oai(path = "/tasks/:id", method = "delete")]
    pub async fn task_delete(
        &self,
        id: Path<String>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<TaskInfo>> {
        let partition_id = Some(partition_id.0);
        self.status.delete(&id, &partition_id).await?;
        self.tasks.delete(&id, &partition_id).await
    }

    /// 获取全部任务状态
    #[oai(path = "/statuses", method = "get")]
    pub async fn status_get_all(
        &self,
        status: Query<Option<i32>>,
        enabled: Query<Option<bool>>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<Vec<TaskStatusInfo>>> {
        self.status
            .find(
                |r| status_filter(r, &status, &enabled),
                &Some(partition_id.0),
            )
            .await
    }

    /// 获取指定任务状态
    #[oai(path = "/statuses/:id", method = "get")]
    pub async fn status_get(
        &self,
        id: Path<String>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<TaskStatusInfo>> {
        self.status.get(&id, &Some(partition_id.0)).await
    }

    /// 更新指定任务状态
    #[oai(path = "/statuses/:id", method = "put")]
    pub async fn status_put(
        &self,
        id: Path<String>,
        record: Json<TaskStatusInfo>,
        partition_id: Query<String>,
    ) -> Result<CodeResponse<TaskStatusInfo>> {
        self.status.put(&id, record, &Some(partition_id.0)).await
    }
}

/// 店铺招牌过滤器
pub fn status_filter(
    record: &TaskStatusInfo,
    status: &Query<Option<i32>>,
    enabled: &Query<Option<bool>>,
) -> bool {
    if let Some(ref status) = status.0
        && record.status != *status
    {
        return false;
    }
    if let Some(ref enabled) = enabled.0
        && record.enabled != *enabled
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
