use std::marker::PhantomData;
use std::path::PathBuf;

use super::db::*;
use crate::DirVariant;
use crate::interface::*;
use rx_core::log::info;
use rx_core::sys::fs::SortOrder;
use rx_core::{sys::fs, text::*};

//#[derive(Size)]
pub struct DirTable<T> {
    path: PathBuf,
    last_id: DirVariant<RecordId>,
    _p: PhantomData<T>,
}

impl<T: IRecord> DirTable<T> {
    /// 打开表
    pub fn open<S>(db: &DirDb, name: S) -> BoxResult<Self>
    where
        S: AsRef<str>,
    {
        let last_id_name = format!("{}_id", name.as_ref());
        let path = db.table_path(&name);
        fs::ensure_dir_exist(&path)?;

        let mut last_id = DirVariant::open(&db, last_id_name, None)?;
        if !last_id.exist() {
            let max_id = find_max_record_id(&path, 0)?;
            last_id.set(&max_id)?;
            info!("{}: last_id set to {}", name.as_ref(), max_id);
        }

        Ok(DirTable::<T> {
            path,
            last_id,
            _p: PhantomData::<T>,
        })
    }

    /// 加载全部记录
    pub fn load_records<S>(db: &DirDb, table_name: &S) -> BoxResult<Vec<T>>
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

    //type Filter = dyn Fn(&Self::Record) -> bool;

    fn name(&self) -> String {
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
        let id = self.next_id()?;
        self.put(id, record)?;
        Ok(id)
    }

    fn put(&mut self, id: RecordId, record: &mut Self::Record) -> BoxResult<()> {
        record.set_id(id);
        json::save(&record, &self.record_path(id))?;
        let last_id = self.last_id.get_or_default();
        if id > last_id {
            self.last_id.set(&id)?;
        }
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
        find_record_ids(&self.path, min_id)
    }

    fn next_id(&mut self) -> BoxResult<RecordId> {
        let mut id = self.last_id.get_or_default();
        id += 1;
        self.last_id.set(&id)?;
        Ok(id)
    }
}

/// 从路径中查找记录ID
fn find_record_ids(path: &Path, min_id: RecordId) -> BoxResult<Vec<RecordId>> {
    let mut ids = Vec::new();
    let names = fs::file_stems_in(path, &"json", SortOrder::None)?;
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

/// 从路径中查找最大记录ID
fn find_max_record_id(path: &Path, min_id: RecordId) -> BoxResult<RecordId> {
    let mut max_id = 0;
    let names = fs::file_stems_in(path, &"json", SortOrder::None)?;
    for stem in names {
        if let Ok(id) = stem.parse::<RecordId>() {
            if id >= min_id && id > max_id {
                max_id = id;
            }
        }
    }
    Ok(max_id)
}

#[cfg(test)]
mod tests {
    use crate::test::tests::*;
    use path_macro::path;

    use super::*;

    #[test]
    fn tab_works() {
        let dir = "/tmp/test/dirdb1";
        let name = "student";
        let db = DirDb::open(dir).unwrap();
        db.remove_table(name).unwrap();
        let p = path!(dir / name);
        assert!(!p.exists());

        let mut tab = DirTable::open(&db, &"student").unwrap();
        println!("tab.find_ids: {:?}", tab.find_ids(0));
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
