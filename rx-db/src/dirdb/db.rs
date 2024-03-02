use std::path::PathBuf;

use rx_core::fs;
use rx_core::text::*;

use crate::{IRecord, ITable, IVariant, RecordId};

use super::table::*;
use super::variant::*;

//pub type BoxResult<T> = std::io::Result<T>;

pub struct DirDb {
    path: PathBuf,
}

impl DirDb {
    /// 打开数据库
    pub fn open<P>(path: P) -> BoxResult<Self>
        where
            P: AsRef<Path>,
    {
        fs::ensure_dir_exist(&path)?;
        Ok(DirDb {
            path: path.as_ref().to_owned(),
        })
    }

    /// 打开数据库
    pub fn open_name<P, S>(path: &P, name: &S) -> BoxResult<Self>
        where
            P: AsRef<Path>,
            S: AsRef<str>,
    {
        let path = fs::join(&path, &name.as_ref());
        Self::open(&path)
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

    /// 加载数据库变量
    pub fn load_variant<T, S>(&mut self, name: S, default: Option<T>) -> BoxResult<T>
        where
            T: Default + Clone + DeserializeOwned + Serialize,
            S: AsRef<str>,
    {
        DirVariant::open(self, name, default)?.get()
    }

    /// 数据库变量路径
    pub fn variant_path<S>(&self, name: S) -> PathBuf
        where
            S: AsRef<str>,
    {
        let mut path = fs::join(&self.path(), &name.as_ref());
        path.set_extension("json");
        path
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

    /// 数据库表路径
    pub fn table_path<S>(&self, name: S) -> PathBuf
        where
            S: AsRef<str>,
    {
        fs::join(&self.path(), &name.as_ref())
    }

    /// 删除数据库表
    pub fn remove_table<S>(&self, name: S) -> BoxResult<()>
        where
            S: AsRef<str>,
    {
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
    pub fn load_records<T, S, P>(&self, name: S, predicate: P) -> BoxResult<Vec<T>>
        where
            T: IRecord,
            S: AsRef<str>,
            P: Fn(&T) -> bool,
    {
        let mut table = DirTable::<T>::open(self, name)?;
        table.find(0, RecordId::MAX, predicate)
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::*;
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn db_works() {
        let s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let _s3 = { Student::new(3, "Joel") };

        let mut db = DirDb::open(&"/tmp/test/dirdb1").unwrap();

        assert_eq!(db.name(), "dirdb1");

        let mut var = db.open_variant(&"var", None).unwrap();
        var.set(&s1).unwrap();

        let mut tab = db.open_table(&"student").unwrap();
        tab.put(1, &mut s2).unwrap();
    }
}
