use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use rx_core::{fs, text::*};

use crate::interface::*;

use super::db::*;

//#[derive(Size)]
pub struct DirTable<T> {
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T: IRecord> DirTable<T> {
    /// 打开表
    pub fn open<S>(db: &DirDb, name: &S) -> BoxResult<Self>
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

    /// 加载全部记录
    pub fn load_records<S>(db: &DirDb, table_name: &S) -> Result<Vec<T>, <Self as ITable>::Err>
    where
        S: AsRef<str>,
    {
        let mut table = DirTable::<T>::open(&db, table_name).unwrap();
        table.find_all()
    }
    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 记录文件全路径
    fn record_path(&self, id: RecordId) -> PathBuf {
        self.path.join(format!("{}.json", id))
    }
}

impl<T: IRecord> ITable for DirTable<T> {
    type Record = T;

    type Err = Box<dyn std::error::Error>;

    //type Filter = dyn Fn(&Self::Record) -> bool;

    fn name(&self) -> &str {
        fs::file_name(&self.path)
    }

    fn len(&self) -> usize {
        unimplemented!()
    }

    fn exist(&self, id: RecordId) -> bool {
        self.record_path(id).exists()
    }

    fn get(&self, id: RecordId) -> BoxResult<Self::Record> {
        json::load(&self.record_path(id))
    }

    fn post(&mut self, record: &mut Self::Record) -> BoxResult<RecordId> {
        let r = record.get_id();
        let id = match r {
            Some(id) if id > 0 => id,
            _ => self.next_id()?,
        };
        self.put(id, record)?;
        Ok(id)
    }

    fn put(&mut self, id: RecordId, record: &mut Self::Record) -> BoxResult<()> {
        record.set_id(id);
        json::save(&record, &self.record_path(id))?;
        Ok(())
    }

    fn delete(&mut self, id: RecordId) -> BoxResult<()> {
        Ok(fs::remove(&self.record_path(id))?)
    }

    fn find<P>(
        &mut self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<Self::Record>>
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
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<(RecordId, Self::Record)>>
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

    fn find_ids(&mut self, min_id: RecordId) -> BoxResult<Vec<RecordId>> {
        let mut ids = Vec::new();
        let names = fs::file_stems_in(&self.path, &"json")?;
        for stem in names {
            if let Ok(id) = stem.parse::<RecordId>() {
                if id >= min_id {
                    ids.push(id);
                }
            }
        }
        ids.sort();
        Ok(ids)
    }

    fn next_id(&mut self) -> BoxResult<RecordId> {
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

        let mut s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let mut s3 = { Student::new(3, "Joel") };

        let id1 = tab.post(&mut s1).unwrap();
        assert_eq!(tab.get(id1).unwrap(), s1);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1]);

        let id2 = tab.post(&mut s2).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s2);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        tab.put(id2, &mut s3).unwrap();
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
