/// 数据库结果
pub use std::result::Result;

/// 数据库表
pub trait ITable {
    /// 记录类型
    type Record: Default;

    /// ID类型
    type Id: Default + Copy;

    /// 错误类型
    type Err;

    /// 过滤条件类型
    //type Filter: ?Sized; //: Default = Fn(&Self::Record) -> bool;
    //type Filter: ?Sized;

    /// 获取表名
    fn name(&self) -> &str;

    /// 获取表长度
    fn len(&mut self) -> usize;

    /// 获取表是否为空
    fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    /// 判断记录是否存在
    fn exist(&mut self, id: Self::Id) -> bool;

    /// 获取记录
    fn get(&mut self, id: Self::Id) -> Result<Self::Record, Self::Err>;

    /// 获取变量值/缺省值
    fn get_or(&mut self, id: Self::Id, record: Self::Record) -> Self::Record {
        self.get(id).unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&mut self, id: Self::Id) -> Self::Record {
        self.get_or(id, Self::Record::default())
    }

    /// 添加记录
    fn post(&mut self, record: &Self::Record) -> Result<Self::Id, Self::Err>;

    /// 更新记录
    fn put(&mut self, id: Self::Id, record: &Self::Record) -> Result<(), Self::Err>;

    /// 删除记录(幂等)
    fn delete(&mut self, id: Self::Id) -> Result<(), Self::Err>;

    /// 查询记录集
    fn find<P>(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        predicate: P,
    ) -> Result<Vec<Self::Record>, Self::Err>
    where
        P: Fn(&Self::Record) -> bool;

    /// 查询记录集
    fn find_all(&mut self) -> Result<Vec<Self::Record>, Self::Err> {
        self.find(Self::Id::default(), usize::max_value(), |_| true)
    }

    /// 查询K/V对
    fn find_pairs<P>(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        predicate: P,
    ) -> Result<Vec<(Self::Id, Self::Record)>, Self::Err>
    where
        P: Fn(&Self::Record) -> bool;

    /// 查询K/V对
    fn find_all_pairs(&mut self) -> Result<Vec<(Self::Id, Self::Record)>, Self::Err> {
        self.find_pairs(Self::Id::default(), usize::max_value(), |_| true)
    }

    /// 查询Id集
    fn find_ids(&mut self, min_id: Self::Id) -> Result<Vec<Self::Id>, Self::Err>;

    /// 获取下一个ID
    fn next_id(&mut self) -> Result<Self::Id, Self::Err>;
}

/// 数据库变量
pub trait IVariant {
    /// 记录类型
    type Record: Default;

    /// 错误类型
    type Err;

    /// 获取变量名
    fn name(&self) -> &str;

    /// 判断变量是否存在
    fn exist(&mut self) -> bool;

    /// 获取变量值
    fn get(&mut self) -> Result<Self::Record, Self::Err>;

    /// 获取变量值/缺省值
    fn get_or(&mut self, record: Self::Record) -> Self::Record {
        self.get().unwrap_or(record)
    }

    /// 获取变量值/缺省值
    fn get_or_default(&mut self) -> Self::Record {
        self.get_or(Self::Record::default())
    }

    /// 设置变量值
    fn set(&mut self, record: &Self::Record) -> Result<(), Self::Err>;
}

/*
/// 数据库
pub trait IDatabase {
    /// 数据库表类型
    type Table: ITable; //TODO: Table是可变泛型，不能在这里确定，可能要把ITable改成泛型形式才可以

    /// 数据库变量类型
    type Variant: IVariant;

    /// 数据库错误类型
    type Err;

    /// 打开数据库表
    fn open_table<T, S>(&mut self, name: S) -> Result<Self::Table, Self::Err>
    where
        S: AsRef<str>;

    /// 删除数据库表/变量
    fn remove<S>(&self, name: S) -> Result<(), Self::Err>
    where
        S: AsRef<str>;

    /// 打开数据库变量
    fn open_variant<T, S>(&mut self, name: S) -> Result<Self::Variant, Self::Err>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>;

    /// 加载数据库变量
    fn load_variant<T, S>(&mut self, name: S) -> Result<T, Self::Err>
        where
            T: Default + DeserializeOwned + Serialize,
            S: AsRef<str>,
    {
        let mut v = self.open_variant(name)?;
        v.get()
    }
}
*/
