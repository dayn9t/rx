//use std::io;

/// 数据库结果
pub use std::io::Result;

/// 数据库表
pub trait Table {
    /// 记录类型
    type Record;

    /// ID类型
    type Id: Default + Copy;

    /// 过滤条件类型
    //type Filter: ?Sized; //: Default = Fn(&Self::Record) -> bool;
    //type Filter: ?Sized;

    /// 获取表名
    fn name(&self) -> &str;

    /// 判断记录是否存在
    fn exist(&self, id: Self::Id) -> bool;

    /// 获取记录
    fn get(&self, id: Self::Id) -> Result<Self::Record>;

    /// 添加记录
    fn post(&mut self, record: &Self::Record) -> Result<Self::Id>;

    /// 更新记录
    fn put(&mut self, id: Self::Id, record: &Self::Record) -> Result<()>;

    /// 删除记录(幂等)
    fn delete(&mut self, id: Self::Id) -> Result<()>;

    /// 查询记录集
    fn find(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Result<Vec<Self::Record>>;

    /// 查询记录集
    fn find_all(&self) -> Result<Vec<Self::Record>> {
        self.find(Self::Id::default(), usize::max_value(), &|_| true)
    }

    /// 查询K/V对
    fn find_pairs(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Result<Vec<(Self::Id, Self::Record)>>;

    /// 查询Id集
    fn find_ids(&self, min_id: Self::Id) -> Result<Vec<Self::Id>>;

    /// 获取下一个ID
    fn next_id(&self) -> Result<Self::Id>;
}

/// 数据库变量
pub trait Variant {
    /// 记录类型
    type Record: Default;

    /// 获取变量名
    fn name(&self) -> &str;

    /// 判断变量是否存在
    fn exist(&self) -> bool;

    /// 获取变量值
    fn get(&self) -> Result<Self::Record>;

    /// 获取变量值/缺省值
    fn get_or(&self, record: Self::Record) -> Self::Record {
        self.get().unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&self) -> Self::Record {
        self.get_or(Self::Record::default())
    }

    /// 设置变量值
    fn set(&mut self, record: &Self::Record) -> Result<()>;
}

/*

// 数据库
pub trait Db {
    // 打开表
    fn open_table<T, S>(&mut self, s: S) -> Box<dyn Table<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
        S: AsRef<str>;

    // 删除表
    fn remove_table<S: AsRef<str>>(&mut self, s: S);

    // 打开变量
    fn open_variant<T>(&mut self) -> dyn Variant<T>
    where
        T: DeserializeOwned + Serialize;

    // 删除变量
    fn remove_variant<S>(&mut self, s: S)
    where
        S: AsRef<str>;
}
*/
