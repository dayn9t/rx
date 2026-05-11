use crate::{IRecord, ITableDyn, IVariant, vec_to_map};
use rx_core::prelude::*;
use std::collections::HashMap;
use std::fmt;

/// 通用错误类型，包含http状态码
#[derive(Debug)]
pub struct HttpError {
    pub code: u16,       // http状态码
    pub message: String, // 错误信息
}

impl HttpError {
    pub fn not_found(message: String) -> Self {
        Self { code: 404, message }
    }
    pub fn bad_request(message: String) -> Self {
        Self { code: 400, message }
    }
    pub fn internal_server_error(message: String) -> Self {
        Self { code: 500, message }
    }
}
impl std::error::Error for HttpError {}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

/// 数据库
pub trait IDatabase {
    /// 打开数据库
    fn open(db_url: &str) -> AnyResult<Self>
    where
        Self: Sized;

    /// 删除数据库变量
    fn remove_variant(&self, variant_name: &str) -> AnyResult<()>;

    /// 获取数据库变量
    fn get_variant<T>(&self, variant_name: &str) -> AnyResult<T>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static,
    {
        self.open_variant(variant_name)?.get()
    }

    /// 设置数据库变量
    fn set_variant<T>(&self, variant_name: &str, value: &T) -> AnyResult<()>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static,
    {
        let mut v = self.open_variant(variant_name)?;
        v.set(value)
    }

    /// 打开数据库变量
    fn open_variant<T>(&self, variant_name: &str) -> AnyResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static,
    {
        self.open_variant_with_default(variant_name, T::default())
    }

    /// 打开数据库变量 - 指定默认值
    fn open_variant_with_default<T>(
        &self,
        variant_name: &str,
        default: T,
    ) -> AnyResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static;

    /// 删除数据库表
    fn remove_table(&self, table_name: &str) -> AnyResult<()>;

    /// 打开数据库表
    fn open_table<R: IRecord + 'static>(
        &self,
        table_name: &str,
    ) -> AnyResult<Box<dyn ITableDyn<R>>>;

    /// 加载数据库表所有记录
    fn load_records<R: IRecord>(
        &self,
        table_name: &str,
        partition_id: &Option<String>,
    ) -> AnyResult<Vec<R>> {
        self.find_records(table_name, |_| true, partition_id)
    }

    /// 查找数据库表所有记录
    fn load_record_map<R: IRecord>(
        &self,
        table_name: &str,
        partition_id: &Option<String>,
    ) -> AnyResult<HashMap<R::RecordId, R>> {
        let rs = self.load_records(table_name, partition_id)?;
        Ok(vec_to_map(rs))
    }

    /// 从数据库表中查找记录集合
    fn find_records<R, P>(
        &self,
        table_name: &str,
        predicate: P,
        partition_id: &Option<String>,
    ) -> AnyResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool;

    /// 从数据库表中过滤满足条件的记录集合
    fn filter_records<R, P>(
        &self,
        table_name: &str,
        predicate: P,
        partition_id: &Option<String>,
    ) -> AnyResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool,
    {
        self.find_records(table_name, predicate, partition_id)
    }

    /// 从数据库表获取指定记录
    fn get_record<R: IRecord + 'static>(&self, table_name: &str, id: &R::RecordId) -> AnyResult<R> {
        self.open_table(table_name)?.get(id, &None)
    }

    /// 更新数据库表中指定记录
    fn put_record<R: IRecord + 'static>(
        &self,
        table_name: &str,
        id: &R::RecordId,
        record: &mut R,
        partition_id: &Option<String>,
    ) -> AnyResult<()> {
        self.open_table(table_name)?.put(id, record, partition_id)
    }
}
