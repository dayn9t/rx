use super::dirdb::DirDb;
use crate::interface::*;
use rx::{fs, text::*};
use std::fs::{remove_file, File};
use std::path::{Path, PathBuf};

pub struct DirTable<T> {
    path: PathBuf,
    // data: T,
    data: Vec<T>,
}

impl<T: Clone + Serialize + DeserializeOwned> DirTable<T> {
    // 打开表
    pub fn open<S: AsRef<str>>(db: &DirDb, name: S) -> Result<Self> {
        let p = PathBuf::from(db.path.join(Path::new(name.as_ref())));
        match fs::ensure_dir_exist(&p) {
            // let v = T;
            Ok(_) => Ok(DirTable {
                path: p,
                data: vec![],
            }),
            Err(err) => Err(err),
        }
    }

    /// 全路径
    fn full_path(&self, id: Id) -> PathBuf {
        self.path.join(format!("{}", id))
    }
}

impl<T: Clone + Serialize + DeserializeOwned> Table<T> for DirTable<T> {
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
        let r: Result<Record<T>> = load_json(&self.full_path(id));
        //r.into()
        None
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
