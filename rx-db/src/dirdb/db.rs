use crate::{IDatabase, IRecord, ITableDyn, IVariant, RecordId};
use anyhow::anyhow;
use path_macro::path;
use rx_core::sys::fs;
use rx_core::text::*;
use std::path::PathBuf;
use url::Url;

pub const SCHEME: &str = "jddb";
pub const EXT: &str = "json";

/// ID变量名
pub fn id_var_name<S>(name: S) -> String
where
    S: AsRef<str>,
{
    format!("{}_id", name.as_ref())
}

/// 获取数据库对象路径，从URL和表名解析路径
pub fn dbo_path(db_url: &str, name: &str) -> BoxResult<PathBuf> {
    let uri = Url::parse(db_url)?;
    if uri.scheme() != SCHEME {
        return Err(anyhow!("Invalid scheme"));
    }
    let path = path!(uri.path() / name);
    Ok(path.into())
}

/// 获取数据库对象路径，从URL和表名解析路径
pub fn db_path(db_url: &str) -> BoxResult<PathBuf> {
    let uri = Url::parse(db_url)?;
    if uri.scheme() != SCHEME {
        return Err(anyhow!("Invalid scheme"));
    }
    Ok(path!(uri.path()))
}

/// 数据库变量路径
pub fn variant_path(db_url: &str, name: &str) -> BoxResult<PathBuf> {
    let path = dbo_path(db_url, name)?;
    Ok(path.with_extension(EXT))
}

pub struct DirDb {
    path: PathBuf,
}

impl IDatabase for DirDb {
    fn open(db_url: &str) -> BoxResult<Self>
    where
        Self: Sized,
    {
        let path = db_path(db_url)?;
        fs::ensure_dir_exist(&path)?;
        Ok(DirDb { path })
    }

    fn remove_variant(&self, variant_name: &str) -> BoxResult<()> {
        let path = self.variant_path(variant_name);
        if path.exists() && !path.is_file() {
            return Err(anyhow!("Table path not dir"));
        }
        Ok(fs::remove(&path)?)
    }

    fn open_variant_with_default<T>(
        &self,
        variant_name: &str,
        default: T,
    ) -> BoxResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize,
    {
        todo!()
    }

    fn remove_table(&self, table_name: &str) -> BoxResult<()> {
        let path = self.table_path(table_name);
        if path.exists() && !path.is_dir() {
            return Err(anyhow!("Table path not dir"));
        }
        Ok(fs::remove(&path)?)
    }

    fn open_table<R: IRecord>(&self, table_name: &str) -> BoxResult<Box<dyn ITableDyn<R>>> {
        todo!()
    }

    fn find_records<R, P>(
        &self,
        table_name: &str,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool,
    {
        todo!()
    }
}
impl DirDb {
    /// 表路径
    pub fn table_path(&self, table_name: &str) -> PathBuf {
        path!(self.path / table_name)
    }

    /// 变量路径
    pub fn variant_path(&self, variant_name: &str) -> PathBuf {
        path!(self.path / format!("{variant_name}.{EXT}"))
    }

    /// 数据库名称
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    /// 数据库路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 打开数据库变量
    pub fn open_variant<T, S>(&mut self, name: S, default: Option<T>) -> BoxResult<DirVariant<T>>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        DirVariant::open(self, name, default)
    }

    /// 打开数据库变量
    pub fn open_variant_default<T, S>(&mut self, name: S) -> BoxResult<DirVariant<T>>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        DirVariant::open(self, name, Some(T::default()))
    }

    /// 加载数据库变量
    pub fn load_variant<T, S>(&mut self, name: S, default: Option<T>) -> BoxResult<T>
    where
        T: Default + Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        DirVariant::open(self, name, default)?.get()
    }

    /// 删除数据库变量
    pub fn remove_variant<S>(&self, name: S) -> BoxResult<()>
    where
        S: AsRef<str>,
    {
        Ok(fs::remove(&self.variant_path(name))?)
    }

    /// 打开数据库表
    pub fn open_table<T, S>(&mut self, name: S) -> BoxResult<DirTable<T>>
    where
        T: IRecord,
        S: AsRef<str>,
    {
        DirTable::open(self, name)
    }

    /// 删除数据库表
    pub fn remove_table<S>(&self, name: &str) -> BoxResult<()> {
        self.remove_variant(&id_var_name(name.as_ref()))?;
        Ok(fs::remove(&self.table_path(name))?)
    }

    /*
        fn find<P>(
            &mut self,
            min_id: RecordId,
            limit: usize,
            predicate: P,
        ) -> Result<Vec<Self::Record>, Self::Err>
        where
            P: Fn(&Self::Record) -> bool;
    */
    /// 加载表中的记录
    pub fn load_records<T, S, P>(&self, name: &str, predicate: P) -> BoxResult<Vec<T>>
    where
        T: IRecord,
        P: Fn(&T) -> bool,
    {
        let mut table = DirTable::<T>::open(self, name)?;
        table.find(0, RecordId::MAX, predicate)
    }

    /// 加载表中的记录
    pub fn load_all_records<T, S>(&self, name: S) -> BoxResult<Vec<T>>
    where
        T: IRecord,
        S: AsRef<str>,
    {
        self.load_records(name, |_: &T| true)
    }
}

/*

/// 打开数据库，加入记录
pub fn db_put_record<T, S>(
    db_path: &Path,
    table_name: S,
    id: RecordId,
    record: &mut T,
) -> BoxResult<()>
where
    T: IRecord,
    S: AsRef<str>,
{
    let db = DirDb::open(&db_path)?;
    let mut table = DirTable::<T>::open(&db, table_name)?;
    table.put(id, record)
}

#[cfg(test)]
mod tests {
    use crate::db::*;
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn db_works() {
        let s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let _s3 = { Student::new(3, "Joel") };

        let mut db = DirDb::open(&"/tmp/test/dir_db1").unwrap();

        assert_eq!(db.name(), "dir_db1");

        let mut var = db.open_variant(&"var", None).unwrap();
        var.set(&s1).unwrap();

        let mut tab = db.open_table(&"student").unwrap();
        tab.put(1, &mut s2).unwrap();
    }
}
*/
