
pub use rustc_serialize::{Encodable, Decodable, json};

// use std::time::SystemTime;
use std::io;

// pub struct Error {
// pub repr: String,
// }
//
// pub enum Repr {
// Os(i32), /*    Simple(ErrorKind),
//    Custom(Box<Custom>), */
// }
//
// pub type Result<T> = result::Result<T, Error>;
//
pub type Result<T> = io::Result<T>;

pub type Id = u64;

// 记录
#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct Record<T: Clone + Decodable + Encodable> {
    pub id: Id,
    // pub creation: SystemTime,
    // pub modification: SystemTime,
    pub data: T,
}

/// 记录集合
pub type Records<T> = Vec<Record<T>>;

/// 记录过滤器
pub type Filter<T> = Fn(&Record<T>) -> bool;

/// 数据库表
pub trait Table<T: Clone + Decodable + Encodable> {
    /// 获取表名
    fn name(&self) -> String;

    /// 判断记录是否存在
    fn exist(&self, id: Id) -> bool;

    /// 查找记录
    fn find(&self, id: Id) -> Option<Record<T>>;

    /// 查询记录集
    fn query(&self, min_id: Id, limit: usize, filter: &Filter<T>) -> Records<T>;

    /// 添加记录
    fn add(&mut self, data: &T) -> Id;

    /// 更新记录
    fn update(&mut self, id: Id, data: &T);

    /// 删除记录(幂等)
    fn remove(&mut self, id: Id);
}

/// 数据库变量
pub trait Variant<T: Decodable + Encodable> {
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

// 数据库
pub trait Db {
    // 打开表
    fn open_table<T, S>(&mut self, s: S) -> Box<Table<T>>
        where T: Clone + Decodable + Encodable,
              S: AsRef<str>;

    // 删除表
    fn remove_table<S: AsRef<str>>(&mut self, s: S);

    // 打开变量
    fn open_variant<T>(&mut self) -> Variant<T> where T: Decodable + Encodable;

    // 删除变量
    fn remove_variant<S>(&mut self, s: S) where S: AsRef<str>;
}
