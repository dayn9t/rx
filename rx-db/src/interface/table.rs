use crate::Deserialize;
use rx_core::text::BoxResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// 记录ID类型
pub type RecordId = usize;

/// 带有Id的记录
pub trait IRecord: Default + Serialize + DeserializeOwned + Sized {
    fn get_id(&self) -> Option<RecordId>;
    fn set_id(&mut self, id: RecordId);
}
/// Vec<Record> => HasMap<ID, Record>
pub fn vec_to_map<R: IRecord>(rs: Vec<R>) -> HashMap<RecordId, R> {
    rs.into_iter().map(|r| (r.get_id().unwrap(), r)).collect()
}
/// 表元数据
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TableMeta {
    pub last_id: RecordId,
}

impl TableMeta {
    pub fn new1() -> Self {
        Self { last_id: 1 }
    }
}

/// 数据库表
pub trait ITableDyn<T: IRecord> {
    /// 打开表
    fn open(db_url: &str, table_name: &str) -> BoxResult<Self>
    where
        Self: Sized;

    //// 删除表
    //fn remove(db_url: &str, table_name: &str) -> BoxResult<()>;

    /// 获取表名
    fn name(&self) -> String;

    /// 获取表长度
    fn len(&self) -> usize {
        self.find_ids(0).unwrap().len()
    }

    /// 获取表是否为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 获取表元数据
    fn get_meta(&self) -> BoxResult<TableMeta>;

    /// 设置表元数据，TODO: 禁止外部调用
    fn set_meta(&mut self, meta: &TableMeta) -> BoxResult<()>;

    /// 更新最后ID，TODO: 禁止外部调用
    fn update_last_id(&mut self, id: RecordId) -> BoxResult<()> {
        let mut meta = self.get_meta()?;
        if id > meta.last_id {
            meta.last_id = id;
            self.set_meta(&meta)
        } else {
            Ok(())
        }
    }

    /// 获取下一个ID
    fn next_id(&mut self) -> BoxResult<RecordId> {
        let mut meta = self.get_meta()?;
        meta.last_id += 1;
        self.set_meta(&meta)?;
        Ok(meta.last_id)
    }

    /// 判断表中是否包含指定ID的记录
    fn contains(&self, id: RecordId) -> bool;

    /// 获取记录
    fn get(&self, id: RecordId) -> BoxResult<T>;

    /// 获取变量值/缺省值
    fn get_or(&self, id: RecordId, record: T) -> T {
        self.get(id).unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self, id: RecordId) -> T {
        self.get_or(id, T::default())
    }

    /// 添加记录
    fn post(&mut self, record: &mut T) -> BoxResult<RecordId> {
        let id = self.next_id()?;
        self.put(id, record)?;
        Ok(id)
    }
    /// 更新记录
    fn put(&mut self, id: RecordId, record: &mut T) -> BoxResult<()>;

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
    fn find_all(&self) -> BoxResult<Vec<T>>;

    /// 查询K/V对
    fn find_all_pairs(&self) -> BoxResult<Vec<(RecordId, T)>>;

    /// 查询Id集
    fn find_ids(&self, min_id: RecordId) -> BoxResult<Vec<RecordId>>;
}

/// 数据库表
pub trait ITable<T: IRecord>: ITableDyn<T> {
    /// 查询记录集
    fn find<P>(&self, min_id: RecordId, limit: usize, predicate: P) -> BoxResult<Vec<T>>
    where
        P: Fn(&T) -> bool,
    {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id)?;
        for id in ids {
            let r = self.get(id)?;
            if predicate(&r) {
                vec.push(r);
                if vec.len() >= limit {
                    break;
                }
            }
        }
        Ok(vec)
    }

    /// 查询K/V对
    fn find_pairs<P>(
        &self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<(RecordId, T)>>
    where
        P: Fn(&T) -> bool,
    {
        let records = self.find(min_id, limit, predicate)?;
        let pairs = records
            .into_iter()
            .map(|record| (record.get_id().unwrap(), record))
            .collect();
        Ok(pairs)
    }
}
