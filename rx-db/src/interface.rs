use rx::text::*;
use std::io;

/// 数据库结果
pub type Result<T> = io::Result<T>;

/// 数据库表
pub trait Table {
    /// 记录类型
    type Record;

    /// ID类型
    type Id: Copy;

    /// 过滤条件类型
    //type Filter: ?Sized; //: Default = Fn(&Self::Record) -> bool;
    //type Filter: ?Sized;

    /// 获取表名
    fn name(&self) -> String;

    /// 判断记录是否存在
    fn exist(&self, id: Self::Id) -> bool;

    /// 获取记录
    fn get(&self, id: Self::Id) -> Option<Self::Record>;

    /// 添加记录
    fn add(&mut self, record: Self::Record) -> Self::Id;

    /// 更新记录
    fn update(&mut self, id: Self::Id, record: Self::Record);

    /// 删除记录(幂等)
    fn remove(&mut self, id: Self::Id);

    /// 查询记录集
    fn find(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<Self::Record>;

    /// 查询Id集
    fn find_id(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<Self::Id>;

    /// 查询K/V对
    fn find_pair(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<(Self::Id, Self::Record)>;
}

/// 数据库变量
pub trait Variant<T: DeserializeOwned + Serialize> {
    /// 获取变量名
    fn name(&self) -> String;

    /// 判断变量是否存在
    fn exist(&self) -> bool;

    /// 获取变量值
    fn get(&self) -> Option<T>;

    /// 获取变量值/缺省值
    fn get_or(&self, v: &T) -> T;

    /// 设置变量值
    fn set(&self, data: &T);
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
