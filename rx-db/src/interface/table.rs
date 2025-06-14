use crate::Deserialize;
use rx_core::text::AnyResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// 记录ID类型
pub type RecordId = usize;

/// 带有Id的记录
pub trait IRecord: Default + Serialize + DeserializeOwned + Sized {
    fn get_id(&self) -> Option<RecordId>;
    fn set_id(&mut self, id: RecordId);

    fn get_partition_id(&self) -> Option<u32> {
        None
    }
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
    fn open(db_url: &str, table_name: &str) -> AnyResult<Self>
    where
        Self: Sized;

    //// 删除表
    //fn remove(db_url: &str, table_name: &str) -> AnyResult<()>;

    /// 获取表名
    fn name(&self) -> String;

    /// 获取表长度
    fn len(&self) -> usize {
        self.find_ids(0, None).unwrap().len()
    }

    /// 获取表是否为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 获取表元数据
    fn get_meta(&self) -> AnyResult<TableMeta>;

    /// 设置表元数据，TODO: 禁止外部调用
    fn set_meta(&mut self, meta: &TableMeta) -> AnyResult<()>;

    /// 更新最后ID，TODO: 禁止外部调用
    fn update_last_id(&mut self, id: RecordId) -> AnyResult<()> {
        let mut meta = self.get_meta()?;
        if id > meta.last_id {
            meta.last_id = id;
            self.set_meta(&meta)
        } else {
            Ok(())
        }
    }

    /// 获取下一个ID
    fn next_id(&mut self) -> AnyResult<RecordId> {
        let mut meta = self.get_meta()?;
        meta.last_id += 1;
        self.set_meta(&meta)?;
        Ok(meta.last_id)
    }

    /// 判断表中是否包含指定ID的记录
    fn contains(&self, id: RecordId) -> bool;

    /// 获取记录
    fn get(&self, id: RecordId) -> AnyResult<T>;

    /// 获取变量值/缺省值
    fn get_or(&self, id: RecordId, record: T) -> T {
        self.get(id).unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self, id: RecordId) -> T {
        self.get_or(id, T::default())
    }

    /// 添加记录
    fn post(&mut self, record: &mut T) -> AnyResult<RecordId> {
        let id = self.next_id()?;
        self.put(id, record)?;
        Ok(id)
    }
    /// 更新记录
    fn put(&mut self, id: RecordId, record: &mut T) -> AnyResult<()>;

    /// 删除记录(幂等)
    fn delete(&mut self, id: RecordId) -> AnyResult<()>;

    /// 删除全部记录(幂等)
    fn delete_all(&mut self, partition_id: Option<u32>) -> AnyResult<()> {
        let ids = self.find_ids(RecordId::default(), partition_id)?;
        for id in ids {
            self.delete(id)?;
        }
        Ok(())
    }

    /// 查询记录集
    fn find_all(&self, partition_id: Option<u32>) -> AnyResult<Vec<T>>;

    /// 查询K/V对
    fn find_all_pairs(&self, partition_id: Option<u32>) -> AnyResult<Vec<(RecordId, T)>>;

    /// 查询Id集
    fn find_ids(&self, min_id: RecordId, partition_id: Option<u32>) -> AnyResult<Vec<RecordId>>;
}

/// 数据库表
pub trait ITable<T: IRecord>: ITableDyn<T> {
    /// 查询记录集
    fn find<P>(
        &self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
        partition_id: Option<u32>,
    ) -> AnyResult<Vec<T>>
    where
        P: Fn(&T) -> bool,
    {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id, partition_id)?;
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
        partition_id: Option<u32>,
    ) -> AnyResult<Vec<(RecordId, T)>>
    where
        P: Fn(&T) -> bool,
    {
        let records = self.find(min_id, limit, predicate, partition_id)?;
        let pairs = records
            .into_iter()
            .map(|record| (record.get_id().unwrap(), record))
            .collect();
        Ok(pairs)
    }
}
