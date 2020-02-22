use std::path::{Path, PathBuf};
use std::fs::{File, remove_file};
use std::io::Read;
use db::*;
use fs;
use super::dirdb::DirDb;

pub struct DirTable<T: Clone + Decodable + Encodable> {
    path: PathBuf, // data: T,
    data: Vec<T>,
}


impl<T: Clone + Decodable + Encodable> DirTable<T> {
    // 打开表
    pub fn open<S: AsRef<str>>(db: &DirDb, name: S) -> Result<Self> {
        let p = PathBuf::from(db.path.join(Path::new(name.as_ref())));
        match fs::ensure_dir_exist(&p) {
            // let v = T;
            Ok(_) => {
                Ok(DirTable {
                    path: p,
                    data: vec![],
                })
            }
            Err(err) => Err(err),
        }
    }

    /// 全路径
    fn full_path(&self, id: Id) -> PathBuf {
        self.path.join(format!("{}", id))
    }
}


impl<T: Clone + Decodable + Encodable> Table<T> for DirTable<T> {
    // 获取表名
    fn name(&self) -> String {
        // self.path.file_name().unwrap().into_string().unwrap()
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    // 判断记录是否存在
    fn exist(&self, id: Id) -> bool {
        self.full_path(id).exists()
    }

    // 查找记录
    fn find(&self, id: Id) -> Option<Record<T>> {
        let mut f = try_or_none!(File::open(&self.full_path(id)));
        let mut s = String::new();
        try_or_none!(f.read_to_string(&mut s));
        let r = try_or_none!(json::decode(&s));
        Some(r)
    }

    // 查询记录集
    fn query(&self, min_id: Id, limit: usize, filter: &Filter<T>) -> Records<T> {
        vec![]
    }

    /// 添加记录
    fn add(&mut self, data: &T) -> Id {
        0
    }

    /// 更新记录
    fn update(&mut self, id: Id, data: &T) {
        File::create(&self.full_path(id));
        // let mut f = try!(File::create("foo.txt"));
        // try!(f.write_all(b"Hello, world!"));
    }

    /// 删除记录(幂等)
    fn remove(&mut self, id: Id) {
        remove_file(self.full_path(id));
    }
}

#[cfg(test)]
mod tests {
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
    }
}
