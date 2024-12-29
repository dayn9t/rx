use rx_core::text::AnyResult;

/// 数据库变量
pub trait IVariant<T: Default + Clone> {
    /// 打开变量，如果不存在则使用默认值
    fn open_with_default(db_url: &str, name: &str, default_value: T) -> AnyResult<Self>
    where
        Self: Sized;

    /// 打开变量
    fn open(db_url: &str, name: &str) -> AnyResult<Self>
    where
        Self: Sized,
    {
        Self::open_with_default(db_url, name, T::default())
    }

    //// 删除变量
    //fn remove(db_url: &str, name: &str) -> AnyResult<()>;

    /// 获取变量名
    fn name(&self) -> &str;

    /// 判断变量是否存在
    fn exist(&self) -> bool;

    /// 获取缺省值
    fn get_default(&self) -> &T;

    /// 获取变量值
    fn get(&self) -> AnyResult<T>;

    /// 获取变量值/缺省值
    fn get_or(&self, record: T) -> T {
        self.get().unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self) -> T {
        self.get_or(T::default())
    }

    /// 设置变量值
    fn set(&mut self, record: &T) -> AnyResult<()>;
}
