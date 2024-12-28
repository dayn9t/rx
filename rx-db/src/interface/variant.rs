use rx_core::text::BoxResult;

/// 数据库变量
pub trait IVariant {
    /// 记录类型
    type Record: Default + Clone;

    /// 打开变量
    fn open(db_url: &str, variant_name: &str) -> BoxResult<Self>
    where
        Self: Sized;

    /// 删除变量
    fn remove(db_url: &str, variant_name: &str) -> BoxResult<()>;

    /// 判定变量是否存在
    fn exists(db_url: &str, variant_name: &str) -> BoxResult<bool>;

    /// 获取变量名
    fn name(&self) -> &str;

    /// 判断变量是否存在
    fn exist(&self) -> bool;

    /// 获取变量值
    fn get(&self) -> BoxResult<Self::Record>;

    /// 获取变量值/缺省值
    fn get_or(&self, record: Self::Record) -> Self::Record {
        self.get().unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self) -> Self::Record {
        self.get_or(Self::Record::default())
    }

    /// 设置变量值
    fn set(&mut self, record: &Self::Record) -> BoxResult<()>;
}
