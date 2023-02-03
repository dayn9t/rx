use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::db::*;
use crate::interface::*;
use rx::{fs, text::*};

//#[derive(Size)]
pub struct DirTable<T> {
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T> DirTable<T> {
    /// 打开表
    pub fn open<S>(db: &DirDb, name: &S) -> IoResult<Self>
    where
        S: AsRef<str>,
    {
        let path = db.table_path(name);
        fs::ensure_dir_exist(&path)?;

        Ok(DirTable::<T> {
            path,
            _p: PhantomData::<T>,
        })
    }

    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 记录文件全路径
    fn record_path(&self, id: usize) -> PathBuf {
        self.path.join(format!("{}.json", id))
    }
}

impl<T: Default + Serialize + DeserializeOwned> ITable for DirTable<T> {
    type Record = T;

    type Id = usize;

    type Err = std::io::Error;

    //type Filter = dyn Fn(&Self::Record) -> bool;

    fn name(&self) -> &str {
        fs::file_name(&self.path)
    }

    fn len(&mut self) -> usize {
        unimplemented!()
    }

    fn exist(&mut self, id: Self::Id) -> bool {
        self.record_path(id).exists()
    }

    fn get(&mut self, id: Self::Id) -> IoResult<Self::Record> {
        load_json(&self.record_path(id))
    }

    fn post(&mut self, record: &Self::Record) -> IoResult<Self::Id> {
        let id = self.next_id()?;
        self.put(id, record)?;
        Ok(id)
    }

    fn put(&mut self, id: Self::Id, record: &Self::Record) -> IoResult<()> {
        save_json(&record, &self.record_path(id))?;
        Ok(())
    }

    fn delete(&mut self, id: Self::Id) -> IoResult<()> {
        fs::remove(&self.record_path(id))
    }

    fn find<P>(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        predicate: P,
    ) -> IoResult<Vec<Self::Record>>
    where
        P: Fn(&Self::Record) -> bool,
    {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id)?;
        for id in ids {
            let r = self.get(id)?;
            if predicate(&r) {
                vec.push(r);
                if vec.len() >= limit {
                    break;
                }
            }
        }
        Ok(vec)
    }

    fn find_pairs<P>(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        predicate: P,
    ) -> IoResult<Vec<(Self::Id, Self::Record)>>
    where
        P: Fn(&Self::Record) -> bool,
    {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id)?;
        for id in ids {
            let r = self.get(id)?;
            if predicate(&r) {
                vec.push((id, r));
                if vec.len() >= limit {
                    break;
                }
            }
        }
        Ok(vec)
    }

    fn find_ids(&mut self, min_id: Self::Id) -> IoResult<Vec<Self::Id>> {
        let mut ids = Vec::new();
        let names = fs::filestems_in(&self.path, &"json")?;
        for stem in names {
            if let Ok(id) = stem.parse::<Self::Id>() {
                if id >= min_id {
                    ids.push(id);
                }
            }
        }
        ids.sort();
        Ok(ids)
    }

    fn next_id(&mut self) -> IoResult<Self::Id> {
        let ids = self.find_ids(0)?;

        let next = if let Some(id) = ids.last() { id + 1 } else { 1 };
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::tests::*;

    #[test]
    fn tab_works() {
        let name = "student";
        let db = DirDb::open(&"/tmp/test/dirdb1").unwrap();
        db.remove_table(name).unwrap();

        let mut tab = DirTable::open(&db, &"student").unwrap();
        assert_eq!(tab.find_ids(0).unwrap().is_empty(), true);

        let s1 = { Student::new(1, "Jack") };
        let s2 = { Student::new(2, "John") };
        let s3 = { Student::new(3, "Joel") };

        let id1 = tab.post(&s1).unwrap();
        assert_eq!(tab.get(id1).unwrap(), s1);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1]);

        let id2 = tab.post(&s2).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s2);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        tab.put(id2, &s3).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s3);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        let all = tab.find_all().unwrap();
        assert_eq!(all, vec![s1.clone(), s3.clone()]);

        let v = tab.find(2, 1, |_| true).unwrap();
        assert_eq!(v, vec![s3.clone()]);

        let name = s1.name.clone();
        let v = tab.find(0, 1, |s| s.name == name).unwrap();
        assert_eq!(v, vec![s1.clone()]);
    }
}
