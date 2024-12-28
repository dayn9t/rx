use crate::dirdb::{DirVariant, EXT, dbo_path};
use crate::{IRecord, ITable, IVariant, RecordId};
use rx_core::sys::fs::SortOrder;
use rx_core::{sys::fs, text::*};
use std::marker::PhantomData;
use std::path::PathBuf;

//#[derive(Size)]
pub struct DirTable<T> {
    name: String,
    path: PathBuf,
    last_id: DirVariant<RecordId>,
    _p: PhantomData<T>,
}

impl<T: IRecord> DirTable<T> {
    /*
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

     */

    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 记录文件全路径
    fn record_path(&self, id: RecordId) -> PathBuf {
        self.path.join(format!("{}.{}", id, EXT))
    }
}

impl<T: IRecord> ITable<T> for DirTable<T> {
    fn open(db_url: &str, name: &str) -> BoxResult<Self> {
        let path = dbo_path(db_url, name)?;
        fs::ensure_dir_exist(&path)?;
        let meta_url = format!("{}/{}/.meta", db_url, name);
        let last_id = DirVariant::open(&meta_url, "last_id")?;
        Ok(DirTable::<T> {
            name: name.to_owned(),
            path,
            last_id,
            _p: PhantomData::<T>,
        })
    }

    fn remove(db_url: &str, table_name: &str) -> BoxResult<()> {
        let path = dbo_path(db_url, table_name)?;
        Ok(fs::remove(&path)?)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn contains(&self, id: RecordId) -> bool {
        self.record_path(id).is_file()
    }

    fn get(&self, id: RecordId) -> BoxResult<T> {
        json::load(&self.record_path(id))
    }

    fn put(&mut self, id: RecordId, record: &mut T) -> BoxResult<()> {
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

    fn find<P>(&self, min_id: RecordId, limit: usize, predicate: P) -> BoxResult<Vec<T>>
    where
        P: Fn(&T) -> bool,
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
        &self,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<(RecordId, T)>>
    where
        P: Fn(&T) -> bool,
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

    fn find_ids(&self, min_id: RecordId) -> BoxResult<Vec<RecordId>> {
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
    let names = fs::file_stems_in(path, EXT, SortOrder::None)?;
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
fn _find_max_record_id(path: &Path, min_id: RecordId) -> BoxResult<RecordId> {
    let mut max_id = 0;
    let names = fs::file_stems_in(path, EXT, SortOrder::None)?;
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

    use super::*;

    #[test]
    fn tab_works() {
        let url = "jddb:///tmp/jddb-test";
        let name = "student1";

        test_table::<DirTable<Student>>(url, name);
    }
}
