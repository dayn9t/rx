use crate::{IRecord, ITableDyn, IVariant, RecordId};
use rx_core::prelude::*;

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

    /// 查找数据库表所有记录
    fn find_all_records<R: IRecord>(&self, table_name: &str) -> AnyResult<Vec<R>> {
        self.find_records(table_name, RecordId::default(), usize::MAX, |_| true)
    }

    /// 从数据库表中查找记录集合
    fn find_records<R, P>(
        &self,
        table_name: &str,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> AnyResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool;

    /// 从数据库表中过滤满足条件的记录集合
    fn filter_records<R, P>(&self, table_name: &str, predicate: P) -> AnyResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool,
    {
        self.find_records(table_name, 0, RecordId::MAX, predicate)
    }

    /// 从数据库表获取指定记录
    fn get_record<R: IRecord + 'static>(&self, table_name: &str, id: RecordId) -> AnyResult<R> {
        self.open_table(table_name)?.get(id)
    }

    /// 更新数据库表中指定记录
    fn put_record<R: IRecord + 'static>(
        &self,
        table_name: &str,
        id: RecordId,
        record: &mut R,
    ) -> AnyResult<()> {
        self.open_table(table_name)?.put(id, record)
    }
}
