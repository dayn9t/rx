use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use super::common::*;
use rx_db::dirdb::DirTable;
use rx_db::{IRecord, ITable, ITableDyn};

//#[derive(Default)]
pub struct DaoList<R: IRecord> {
    table: Mutex<DirTable<R>>,
}

impl<R: IRecord + ToJSON> DaoList<R> {
    /// 打开数据库表
    pub fn open_name(db_path: &FsPath, table_name: &str) -> AnyResult<Self> {
        let tab = DirTable::open_path(db_path, table_name)?;
        let table = Mutex::new(tab);
        Ok(Self { table })
    }

    /// 获取模型对应的表
    pub async fn table(&self) -> MutexGuard<DirTable<R>> {
        self.table.lock().await
    }

    /// 获取记录集合
    pub async fn get_rs(&self, partition: &Option<String>) -> Result<Vec<R>> {
        let tab = self.table.lock().await;
        let rs = tab.find_all(partition)?;
        Ok(rs)
    }

    /// 添加记录
    pub async fn post(
        &self,
        mut record: Json<R>,
        partition_id: &Option<String>,
    ) -> Result<CodeResponse<R>> {
        let mut tab = self.table.lock().await;
        let _id = tab.post(&mut record.0, partition_id)?;
        Ok(CodeResponse::Created(record))
    }

    /// 获取记录
    pub async fn get(
        &self,
        id: &Path<String>,
        partition_id: &Option<String>,
    ) -> Result<CodeResponse<R>> {
        let tab = self.table.lock().await;
        let id = match id.0.parse() {
            Ok(id) => id,
            Err(_) => return Ok(CodeResponse::InvalidRequest),
        };
        match tab.get(&id, partition_id) {
            Ok(r) => Ok(CodeResponse::Ok(Json(r))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    /// 获取记录集合
    pub async fn get_all(&self, partition_id: &Option<String>) -> Result<CodeResponse<Vec<R>>> {
        let tab = self.table.lock().await;
        match tab.find_all(partition_id) {
            Ok(rs) => Ok(CodeResponse::Ok(Json(rs))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    pub async fn find<P>(
        &self,
        limit: usize,
        predicate: P,
        partition_id: &Option<String>,
    ) -> Result<CodeResponse<Vec<R>>>
    where
        P: Fn(&R) -> bool,
    {
        let tab = self.table.lock().await;
        match tab.find(limit, predicate, partition_id) {
            Ok(rs) => Ok(CodeResponse::Ok(Json(rs))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    /// 删除元素
    pub async fn delete(&self, id: &Path<String>) -> Result<CodeResponse<R>> {
        // FIXME: 不返回删除的元素, 好像是poem的BUG
        let mut tab = self.table.lock().await;
        let id = match id.0.parse() {
            Ok(id) => id,
            Err(_) => return Ok(CodeResponse::InvalidRequest),
        };
        tab.delete(&id)?;
        Ok(CodeResponse::NoContent)
    }

    /// 更新元素
    pub async fn update(
        &self,
        id: &Path<String>,
        mut record: Json<R>,
        partition_id: &Option<String>,
    ) -> Result<CodeResponse<R>> {
        let mut tab = self.table.lock().await;
        let id = match id.0.parse() {
            Ok(id) => id,
            Err(_) => return Ok(CodeResponse::InvalidRequest),
        };
        tab.put(&id, &mut record.0, partition_id)?;
        Ok(CodeResponse::Created(record))
    }

    /// 更新元素
    pub async fn update_record(
        &self,
        id: &R::RecordId,
        mut record: R,
        partition_id: &Option<String>,
    ) {
        let mut tab = self.table.lock().await;
        tab.put(id, &mut record, partition_id).unwrap();
    }
}
