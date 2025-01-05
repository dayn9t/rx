use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

use super::common::*;
pub use rx_db::RecordId;
use rx_db::dirdb::DirTable;
use rx_db::{IRecord, ITable, ITableDyn};

//#[derive(Default)]
pub struct DaoList<R> {
    table: Mutex<DirTable<R>>,
}

impl<R: IRecord + ToJSON> DaoList<R> {
    /// 打开数据库表
    pub fn open_name(db_path: &Path, table_name: &str) -> AnyResult<Self> {
        let tab = DirTable::open_path(db_path, table_name).unwrap();
        let table = Mutex::new(tab);
        Ok(Self { table })
    }

    /// 获取模型对应的表
    pub async fn table(&self) -> MutexGuard<DirTable<R>> {
        self.table.lock().await
    }

    /// 获取记录集合
    pub async fn get_rs(&self) -> Result<Vec<R>> {
        let tab = self.table.lock().await;
        let rs = tab.find_all()?;
        Ok(rs)
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
        let tab = self.table.lock().await;
        match tab.find_all() {
            Ok(rs) => Ok(CodeResponse::Ok(Json(rs))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    pub async fn find<P>(
        &self,
        start_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> Result<CodeResponse<Vec<R>>>
    where
        P: Fn(&R) -> bool,
    {
        let tab = self.table.lock().await;
        match tab.find(start_id, limit, predicate) {
            Ok(rs) => Ok(CodeResponse::Ok(Json(rs))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    pub async fn find_page<P>(
        &self,
        page: Option<usize>,
        page_size: Option<usize>,
        predicate: P,
    ) -> Result<CodeResponse<Vec<R>>>
    where
        P: Fn(&R) -> bool,
    {
        // FIXME: 检查分页参数
        let tab = self.table.lock().await;
        match tab.find(0, usize::MAX, predicate) {
            Ok(rs) => Ok(CodeResponse::Ok(Json(get_page(rs, page, page_size)))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    /// 删除元素
    pub async fn delete(&self, id: UrlPath<u64>) -> Result<CodeResponse<R>> {
        // FIXME: 不返回删除的元素, 好像是poem的BUG
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

    /// 更新元素
    pub async fn update_record(&self, id: RecordId, mut record: R) {
        let mut tab = self.table.lock().await;
        tab.put(id as RecordId, &mut record).unwrap();
    }
}

/// 分页
fn get_page<R>(rs: Vec<R>, page: Option<usize>, page_size: Option<usize>) -> Vec<R> {
    if page.is_none() || page_size.is_none() {
        return rs;
    }
    let page = page.unwrap().max(1);
    let page_size = page_size.unwrap().max(1);
    let start = (page - 1) * page_size;
    rs.into_iter().skip(start).take(page_size).collect()
}
