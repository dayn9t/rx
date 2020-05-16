use super::table::*;
use super::variant::*;
//use crate::interface::*;

use rx::fs;
use rx::text::*;
use std::path::{Path, PathBuf};

pub struct DirDb {
    path: PathBuf,
}

impl DirDb {
    /// 打开数据库
    pub fn open<P>(path: &P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        fs::ensure_dir_exist(&path)?;
        Ok(DirDb {
            path: path.as_ref().to_owned(),
        })
    }

    /// 打开数据库
    pub fn open_name<P, S>(path: &P, name: &S) -> Result<Self>
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
    fn open_varient<T, S>(&mut self, name: S) -> Result<DirVarient<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        DirVarient::open(self, name)
    }

    /// 数据库变量路径
    pub fn varient_path<S>(&self, name: S) -> PathBuf
    where
        S: AsRef<str>,
    {
        let mut path = fs::join(&self.path(), &name.as_ref());
        path.set_extension("json");
        path
    }

    /// 删除数据库变量
    pub fn remove_varient<S>(&self, name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        fs::remove(&self.varient_path(name))
    }

    /// 打开数据库表
    fn open_table<T, S>(&mut self, name: &S) -> Result<DirTable<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
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
    pub fn remove_table<S>(&self, name: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        fs::remove(&self.table_path(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_works() {
        let db = DirDb::open(&"/tmp/test/dirdb1").unwrap();

        assert_eq!(db.name(), "dirdb1");
    }
}
