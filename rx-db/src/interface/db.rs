/*
/// 数据库
pub trait IDatabase {
    /// 数据库表类型
    type Table: ITable; //TODO: Table是可变泛型，不能在这里确定，可能要把ITable改成泛型形式才可以

    /// 数据库变量类型
    type Variant: IVariant;

    //// 数据库错误类型
    //type Err;

    /// 打开数据库表
    fn open_table<T, S>(&mut self, name: S) -> BoxResult<Self::Table>
    where
        S: AsRef<str>;

    /// 删除数据库表/变量
    fn remove<S>(&self, name: S) -> BoxResult<()>
    where
        S: AsRef<str>;

    /// 打开数据库变量
    fn open_variant<T, S>(&mut self, name: S) -> BoxResult<Self::Variant>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>;

    /// 加载数据库变量
    fn load_variant<T, S>(&mut self, name: S) -> BoxResult<T>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let mut v = self.open_variant::<T, S>(name)?;
        //v.get()
        todo!("load_variant")
    }
}
*/
