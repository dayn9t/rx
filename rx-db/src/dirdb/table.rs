use crate::dirdb::{DirVariant, EXT, db_path, meta_path};
use crate::{IRecord, ITable, ITableDyn, IVariant, RecordId, TableMeta};
use path_macro::path;
use rx_core::sys::fs::SortOrder;
use rx_core::{sys::fs, text::*};
use std::marker::PhantomData;
use std::path::PathBuf;

//#[derive(Size)]
pub struct DirTable<T> {
    name: String,
    path: PathBuf,
    meta: DirVariant<TableMeta>,
    _p: PhantomData<T>,
}

impl<T: IRecord> DirTable<T> {
    /// 打开表
    pub fn new(name: String, db_path: &Path) -> BoxResult<Self> {
        let path = path!(db_path / name);
        fs::ensure_dir_exist(&path)?;
        let meta = DirVariant::open_path(&meta_path(db_path), &name)?;
        Ok(Self {
            name,
            path,
            meta,
            _p: PhantomData::<T>,
        })
    }

    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// 记录文件全路径
    fn record_path(&self, id: RecordId) -> PathBuf {
        self.path.join(format!("{}.{}", id, EXT))
    }
}

impl<T: IRecord> ITableDyn<T> for DirTable<T> {
    fn open(db_url: &str, name: &str) -> BoxResult<Self> {
        let db_path = db_path(db_url)?;
        Self::new(name.to_owned(), &db_path)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_meta(&self) -> BoxResult<TableMeta> {
        self.meta.get()
    }

    fn set_meta(&mut self, meta: &TableMeta) -> BoxResult<()> {
        self.meta.set(meta)
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
        self.update_last_id(id)
    }

    fn delete(&mut self, id: RecordId) -> BoxResult<()> {
        Ok(fs::remove(&self.record_path(id))?)
    }

    /// 查询记录集
    fn find_all(&self) -> BoxResult<Vec<T>> {
        self.find(RecordId::default(), usize::MAX, |_| true)
    }

    /// 查询K/V对
    fn find_all_pairs(&self) -> BoxResult<Vec<(RecordId, T)>> {
        self.find_pairs(RecordId::default(), usize::max_value(), |_| true)
    }

    fn find_ids(&self, min_id: RecordId) -> BoxResult<Vec<RecordId>> {
        find_record_ids(&self.path, min_id)
    }
}

impl<T: IRecord> ITable<T> for DirTable<T> {}

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
