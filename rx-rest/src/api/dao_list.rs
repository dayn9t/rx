use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

pub use rx_db::RecordId;
use rx_db::{DirDb, DirTable, IRecord, ITable};

use super::common::*;

//#[derive(Default)]
pub struct DaoList<R> {
    table: Mutex<DirTable<R>>,
}

impl<R: IRecord + ToJSON> DaoList<R> {
    /// 打开数据库表
    pub fn open_name<P, S>(db_path: P, table_name: &S) -> BoxResult<Self>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let db = DirDb::open(db_path).unwrap();
        let table = Mutex::new(DirTable::open(&db, table_name).unwrap());
        Ok(Self { table })
    }

    /// 获取模型对应的表
    pub async fn table(&self) -> MutexGuard<DirTable<R>> {
        self.table.lock().await
    }

    /// 添加记录
    pub async fn post(&self, mut record: Json<R>) -> Result<CodeResponse<R>> {
        let mut tab = self.table.lock().await;
        let _id = tab.post(&mut record.0).unwrap();
        Ok(CodeResponse::Created(record))
    }

    /// 获取记录
    pub async fn get(&self, id: UrlPath<u64>) -> Result<CodeResponse<R>> {
        let tab = self.table.lock().await;
        match tab.get(id.0 as RecordId) {
            Ok(r) => Ok(CodeResponse::Ok(Json(r))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    /// 获取记录集合
    pub async fn get_all(&self) -> Result<CodeResponse<Vec<R>>> {
        let mut tab = self.table.lock().await;
        match tab.find_all() {
            Ok(rs) => Ok(CodeResponse::Ok(Json(rs))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    /// 删除元素
    pub async fn delete(&self, id: UrlPath<u64>) -> Result<CodeResponse<R>> {
        let mut tab = self.table.lock().await;
        tab.delete(id.0 as RecordId).unwrap();
        Ok(CodeResponse::NoContent)
    }

    /// 更新元素
    pub async fn update(&self, id: UrlPath<u64>, mut record: Json<R>) -> Result<CodeResponse<R>> {
        let mut tab = self.table.lock().await;
        tab.put(id.0 as RecordId, &mut record.0).unwrap();
        Ok(CodeResponse::Created(record))
    }
}
