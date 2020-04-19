use super::table::*;
use crate::interface::*;
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
        let ok = fs::ensure_dir_exist(&path)?;
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
    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    /// 数据库名称
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 打开表
    fn open_table<T, S>(&mut self, table_name: S) -> Result<DirTable<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        DirTable::open(self, table_name)
    }
}

#[test]
fn it_works() {
    //let db = DirDb::open("db");
}
