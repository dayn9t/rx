use std::path::{Path, PathBuf};
use db::*;
use fs;
use super::table::*;

pub struct DirDb {
    pub path: PathBuf,
}

impl DirDb {
    // 打开数据库
    pub fn open<P: AsRef<Path>>(path: &P) -> Result<Self> {
        match fs::ensure_dir_exist(path) {
            Ok(_) => Ok(DirDb { path: PathBuf::from(path.as_ref()) }),
            Err(err) => Err(err),
        }
    }

    // 打开表
    fn open_table<T, S>(&mut self, table_name: S) -> Result<DirTable<T>>
        where T: Clone + Decodable + Encodable,
              S: AsRef<str>
    {
        DirTable::open(self, table_name)
    }
}

#[test]
fn it_works() {
    let db = DirDb::open("db");
}
