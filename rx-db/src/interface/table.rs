use rx_core::text::BoxResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// 记录ID类型
pub type RecordId = usize;

/// 带有Id的记录
pub trait IRecord: Default + Serialize + DeserializeOwned {
    fn get_id(&self) -> Option<RecordId>;
    fn set_id(&mut self, id: RecordId);
}

/// Vec<Record> => HasMap<ID, Record>
pub fn vec_to_map<R: IRecord>(rs: Vec<R>) -> HashMap<RecordId, R> {
    rs.into_iter().map(|r| (r.get_id().unwrap(), r)).collect()
}

/// 数据库表
pub trait ITable {
    /// 记录类型
    type Record: IRecord;

    /// 打开表
    fn open(db_url: &str, table_name: &str) -> BoxResult<Self>
    where
        Self: Sized;

    /// 删除表
    fn remove(db_url: &str, table_name: &str) -> BoxResult<()>;

    /// 判定表是否存在
    fn exists(db_url: &str, table_name: &str) -> BoxResult<bool>;

    /// 获取表名
    fn name(&self) -> String;

    /// 获取表长度
    fn len(&self) -> usize;

    /// 获取表是否为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 判断表中是否包含指定ID的记录
    fn contains(&self, id: RecordId) -> bool;

    /// 获取记录
    fn get(&self, id: RecordId) -> BoxResult<Self::Record>;

    /// 获取变量值/缺省值
    fn get_or(&mut self, id: RecordId, record: Self::Record) -> Self::Record {
        self.get(id).unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&mut self, id: RecordId) -> Self::Record {
        self.get_or(id, Self::Record::default())
    }

    /// 添加记录
    fn post(&mut self, record: &mut Self::Record) -> BoxResult<RecordId>;

    /// 更新记录
    fn put(&mut self, id: RecordId, record: &mut Self::Record) -> BoxResult<()>;

    /// 删除记录(幂等)
    fn delete(&mut self, id: RecordId) -> BoxResult<()>;

    /// 删除全部记录(幂等)
    fn delete_all(&mut self) -> BoxResult<()> {
        let ids = self.find_ids(RecordId::default())?;
        for id in ids {
            self.delete(id)?;
        }
        Ok(())
    }
    /// 查询记录集
    fn find<P>(
        &mut self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<Self::Record>>
    where
        P: Fn(&Self::Record) -> bool;

    /// 查询记录集
    fn find_all(&mut self) -> BoxResult<Vec<Self::Record>> {
        self.find(RecordId::default(), usize::MAX, |_| true)
    }

    /// 查询K/V对
    fn find_pairs<P>(
        &mut self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<(RecordId, Self::Record)>>
    where
        P: Fn(&Self::Record) -> bool;

    /// 查询K/V对
    fn find_all_pairs(&mut self) -> BoxResult<Vec<(RecordId, Self::Record)>> {
        self.find_pairs(RecordId::default(), usize::max_value(), |_| true)
    }

    /// 查询Id集
    fn find_ids(&mut self, min_id: RecordId) -> BoxResult<Vec<RecordId>>;

    /// 获取下一个ID
    fn next_id(&mut self) -> BoxResult<RecordId>;
}
