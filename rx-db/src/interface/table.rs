use crate::Deserialize;
use rx_core::text::AnyResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

pub trait IRecordId:
    Default
    + Clone
    + Eq
    + PartialEq
    + Ord
    + Hash
    + Debug
    + FromStr
    + ToString
    + Display
    + Serialize
    + DeserializeOwned
{
    fn next(&self) -> Self;
}

impl IRecordId for usize {
    fn next(&self) -> Self {
        self + 1
    }
}

/// 带有Id的记录
pub trait IRecord: Default + Serialize + DeserializeOwned + Sized {
    /// 记录ID类型
    type RecordId: IRecordId;
    fn get_id(&self) -> Option<Self::RecordId>;
    fn set_id(&mut self, id: &Self::RecordId);

    fn get_partition_id(&self) -> Option<String> {
        None
    }
}

/// Vec<Record> => HasMap<ID, Record>
pub fn vec_to_map<R: IRecord>(rs: Vec<R>) -> HashMap<R::RecordId, R> {
    rs.into_iter().map(|r| (r.get_id().unwrap(), r)).collect()
}

/// 表元数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(bound = "RID: Serialize + DeserializeOwned")]
pub struct TableMeta<RID: IRecordId> {
    pub last_id: RID,
}

/// 数据库表
pub trait ITableDyn<R: IRecord> {
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
        self.find_ids(None).unwrap().len()
    }

    /// 获取表是否为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 获取表元数据
    fn get_meta(&self) -> AnyResult<TableMeta<R::RecordId>>;

    /// 设置表元数据，TODO: 禁止外部调用
    fn set_meta(&mut self, meta: &TableMeta<R::RecordId>) -> AnyResult<()>;

    /// 更新最后ID，TODO: 禁止外部调用
    fn update_last_id(&mut self, id: &R::RecordId) -> AnyResult<()> {
        let mut meta = self.get_meta()?;
        if *id > meta.last_id {
            meta.last_id = id.clone();
            self.set_meta(&meta)
        } else {
            Ok(())
        }
    }

    /// 获取下一个ID
    fn next_id(&mut self) -> AnyResult<R::RecordId> {
        let mut meta = self.get_meta()?;
        meta.last_id = meta.last_id.next();
        self.set_meta(&meta)?;
        Ok(meta.last_id)
    }

    /// 判断表中是否包含指定ID的记录
    fn contains(&self, id: &R::RecordId) -> bool;

    /// 获取记录
    fn get(&self, id: &R::RecordId) -> AnyResult<R>;

    /// 获取变量值/缺省值
    fn get_or(&self, id: &R::RecordId, record: R) -> R {
        self.get(id).unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self, id: &R::RecordId) -> R {
        self.get_or(id, R::default())
    }

    /// 添加记录
    fn post(&mut self, record: &mut R) -> AnyResult<R::RecordId> {
        let id = match record.get_id() {
            None => self.next_id()?,
            Some(id) => id,
        };

        self.put(&id, record)?;
        Ok(id)
    }
    /// 更新记录
    fn put(&mut self, id: &R::RecordId, record: &mut R) -> AnyResult<()>;

    /// 删除记录(幂等)
    fn delete(&mut self, id: &R::RecordId) -> AnyResult<()>;

    /// 删除全部记录(幂等)
    fn delete_all(&mut self, partition_id: Option<u32>) -> AnyResult<()> {
        let ids = self.find_ids(partition_id)?;
        for id in ids {
            self.delete(&id)?;
        }
        Ok(())
    }

    /// 查询记录集
    fn find_all(&self, partition_id: Option<u32>) -> AnyResult<Vec<R>>;

    /// 查询K/V对
    fn find_all_pairs(&self, partition_id: Option<u32>) -> AnyResult<Vec<(R::RecordId, R)>>;

    /// 查询Id集
    fn find_ids(&self, partition_id: Option<u32>) -> AnyResult<Vec<R::RecordId>>;
}

/// 数据库表
pub trait ITable<R: IRecord>: ITableDyn<R> {
    /// 查询记录集
    fn find<P>(&self, limit: usize, predicate: P, partition_id: Option<u32>) -> AnyResult<Vec<R>>
    where
        P: Fn(&R) -> bool,
    {
        let mut vec = Vec::new();
        let ids = self.find_ids(partition_id)?;
        for id in ids {
            let r = self.get(&id)?;
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
        limit: usize,
        predicate: P,
        partition_id: Option<u32>,
    ) -> AnyResult<Vec<(R::RecordId, R)>>
    where
        P: Fn(&R) -> bool,
    {
        let records = self.find(limit, predicate, partition_id)?;
        let pairs = records
            .into_iter()
            .map(|record| (record.get_id().unwrap(), record))
            .collect();
        Ok(pairs)
    }
}
