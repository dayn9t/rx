use super::dirdb::DirDb;
use crate::interface::*;
use rx::{fs, text::*};
use std::fs::{remove_file, File};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

//#[derive(Size)]
pub struct DirTable<T: Sized> {
    name: String,
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T> DirTable<T> {
    /// 打开表
    pub fn open<S>(db: &DirDb, name: S) -> Result<Self>
        where
            S: AsRef<str>,
    {
        let path = fs::join(&db.path(), &name.as_ref());
        let ok = fs::ensure_dir_exist(&path)?;

        Ok(DirTable::<T> {
            name: name.as_ref().into(),
            path,
            _p: PhantomData::<T>,
        })
    }

    /// 数据库名称
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// 数据库名称
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 全路径
    fn full_path(&self, id: usize) -> PathBuf {
        self.path.join(format!("{}", id))
    }
}

impl<T: Sized + Clone + Serialize + DeserializeOwned> Table for DirTable<T> {
    type Record = T;

    type Id = usize;

    //type Filter = dyn Fn(&Self::Record) -> bool;

    fn name(&self) -> String {
        // self.path.file_name().unwrap().into_string().unwrap()
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn exist(&self, id: Self::Id) -> bool {
        self.full_path(id).exists()
    }

    fn get(&self, id: Self::Id) -> Option<Self::Record> {
        let r = load_json(&self.full_path(id));
        r.ok()
    }

    fn add(&mut self, record: Self::Record) -> Self::Id {
        0
    }

    fn update(&mut self, id: Self::Id, record: Self::Record) {
        File::create(&self.full_path(id));
        // let mut f = try!(File::create("foo.txt"));
        // try!(f.write_all(b"Hello, world!"));
    }

    fn remove(&mut self, id: Self::Id) {
        remove_file(self.full_path(id));
    }

    fn find(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<Self::Record> {
        let mut rs = Vec::new();
        let ids = self.find_id(min_id, limit, filter);
        for id in ids {
            rs.push(self.get(id).unwrap());
        }
        rs
    }

    fn find_id(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<Self::Id> {
        Vec::new()
    }

    fn find_pair(
        &self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> Vec<(Self::Id, Self::Record)> {
        let mut rs = Vec::new();
        let ids = self.find_id(min_id, limit, filter);
        for id in ids {
            rs.push((id, self.get(id).unwrap()));
        }
        rs
    }
}

#[cfg(test)]
mod tests {
    /*
    extern crate test;
    use self::Animal::*;
    use self::test::Bencher;
    use {Encodable, Decodable};


    #[derive(RustcDecodable, Eq, PartialEq, Debug)]
    struct OptionData {
        opt: Option<usize>,
    }

    #[test]
    fn test_decode_option_none() {
        let s = "{}";
        let obj: OptionData = super::decode(s).unwrap();
        assert_eq!(obj, OptionData { opt: None });
    }
    #[test]
    fn it_works() {
        struct Student {
            number: i32,
            name: String,
        }

        let db = DirDb::open("db");
        // let students = db.open_table<Student>("student");
    }*/
}

#[test]
fn it_works() {
    #[derive(Clone, Serialize, Deserialize)]
    struct Student {
        number: i32,
        name: String,
    }
    let s1 = Student {
        number: 1,
        name: "John".to_string(),
    };

    //let rs1 = Record { id: 100, data: s1 };
}
