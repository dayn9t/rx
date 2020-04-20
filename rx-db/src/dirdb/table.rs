use std::fs::{remove_file, File};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use rx::{fs, text::*};

use crate::interface::*;

use super::dirdb::DirDb;

//#[derive(Size)]
pub struct DirTable<T> {
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T> DirTable<T> {
    /// 打开表
    pub fn open<S>(db: &DirDb, name: &S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let path = db.table_path(name);
        let ok = fs::ensure_dir_exist(&path)?;

        Ok(DirTable::<T> {
            path,
            _p: PhantomData::<T>,
        })
    }

    /// 数据库名称
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 全路径
    fn full_path(&self, id: usize) -> PathBuf {
        self.path.join(format!("{}.json", id))
    }
}

impl<T: Serialize + DeserializeOwned> Table for DirTable<T> {
    type Record = T;

    type Id = usize;

    //type Filter = dyn Fn(&Self::Record) -> bool;

    fn name(&self) -> &str {
        fs::file_name(&self.path)
    }

    fn exist(&self, id: Self::Id) -> bool {
        self.full_path(id).exists()
    }

    fn get(&self, id: Self::Id) -> Result<Self::Record> {
        load_json(&self.full_path(id))
    }

    fn add(&mut self, record: Self::Record) -> Result<Self::Id> {
        Ok(0)
    }

    fn update(&mut self, id: Self::Id, record: Self::Record) -> Result<()> {
        File::create(&self.full_path(id));
        // let mut f = try!(File::create("foo.txt"));
        // try!(f.write_all(b"Hello, world!"));
        Ok(())
    }

    fn remove(&mut self, id: Self::Id) -> Result<()> {
        remove_file(self.full_path(id))
    }

    fn find(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Result<Vec<Self::Record>> {
        let mut rs = Vec::new();
        let ids = self.find_id(min_id, limit, filter)?;
        for id in ids {
            rs.push(self.get(id)?);
        }
        Ok(rs)
    }

    fn find_id(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Result<Vec<Self::Id>> {
        let ids = Vec::new();
        Ok(ids)
    }

    fn find_pair(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Result<Vec<(Self::Id, Self::Record)>> {
        let mut rs = Vec::new();
        let ids = self.find_id(min_id, limit, filter)?;
        for id in ids {
            rs.push((id, self.get(id)?));
        }
        Ok(rs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct Student {
        number: i32,
        name: String,
    }

    #[test]
    fn it_works() {
        let db = DirDb::open(&"/tmp/test/dirdb1").unwrap();
        let mut student_tab = DirTable::open(&db, &"student").unwrap();

        let s1 = Student {
            number: 1,
            name: "John".to_string(),
        };
        let s2 = Student {
            number: 2,
            name: "Jack".to_string(),
        };

        student_tab.add(s1);
        student_tab.add(s2);
    }
}
