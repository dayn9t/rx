use crate::dirdb::{DirVariant, EXT, db_path, meta_path};
use crate::{IRecord, ITable, ITableDyn, IVariant, RecordId, TableMeta};
use anyhow::anyhow;
use path_macro::path;
use rx_core::sys::fs::{SortOrder, find_file_by_name};
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
    pub fn open_path(db_path: &Path, name: &str) -> AnyResult<Self> {
        let path = path!(db_path / name);
        fs::ensure_dir_exist(&path)?;
        let meta = DirVariant::open_path(&meta_path(db_path), &name)?;
        Ok(Self {
            name: name.to_owned(),
            path,
            meta,
            _p: PhantomData::<T>,
        })
    }

    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn record_name(id: RecordId) -> String {
        format!("{}.{}", id, EXT)
    }

    /// 记录文件全路径
    fn record_path(&self, id: RecordId, partition_id: Option<u32>) -> PathBuf {
        let name = Self::record_name(id);
        let name = if let Some(partition_id) = partition_id {
            format!("{}/{}", partition_id, name)
        } else {
            name
        };
        path!(self.path / name)
    }

    /// 查找记录文件全路径
    fn find_record_path(&self, id: RecordId) -> AnyResult<PathBuf> {
        let name = Self::record_name(id);
        let files = find_file_by_name(&self.path, &name).unwrap_or_default();
        if files.is_empty() {
            Err(anyhow!("record not found: {}", id))
        } else if files.len() > 1 {
            Err(anyhow!("duplicate record: {} => {:?}", id, files))
        } else {
            Ok(files[0].clone())
        }
    }
}

impl<T: IRecord> ITableDyn<T> for DirTable<T> {
    fn open(db_url: &str, name: &str) -> AnyResult<Self> {
        let db_path = db_path(db_url)?;
        Self::open_path(&db_path, name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_meta(&self) -> AnyResult<TableMeta> {
        self.meta.get()
    }

    fn set_meta(&mut self, meta: &TableMeta) -> AnyResult<()> {
        self.meta.set(meta)
    }

    fn contains(&self, id: RecordId) -> bool {
        self.find_record_path(id).is_ok()
    }

    fn get(&self, id: RecordId) -> AnyResult<T> {
        let p = self.find_record_path(id)?;
        json::load(&p)
    }

    fn put(&mut self, id: RecordId, record: &mut T) -> AnyResult<()> {
        record.set_id(id);
        let partition_id = record.get_partition_id();
        json::save(&record, &self.record_path(id, partition_id))?;
        self.update_last_id(id)
    }

    fn delete(&mut self, id: RecordId) -> AnyResult<()> {
        let p = self.find_record_path(id)?;
        Ok(fs::remove(&p)?)
    }

    /// 查询记录集
    fn find_all(&self) -> AnyResult<Vec<T>> {
        self.find(RecordId::default(), usize::MAX, |_| true)
    }

    /// 查询K/V对
    fn find_all_pairs(&self) -> AnyResult<Vec<(RecordId, T)>> {
        self.find_pairs(RecordId::default(), usize::max_value(), |_| true)
    }

    fn find_ids(&self, min_id: RecordId) -> AnyResult<Vec<RecordId>> {
        find_record_ids(&self.path, min_id)
    }
}

impl<T: IRecord> ITable<T> for DirTable<T> {}

/// 从路径中查找记录ID
fn find_record_ids(path: &Path, min_id: RecordId) -> AnyResult<Vec<RecordId>> {
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
fn _find_max_record_id(path: &Path, min_id: RecordId) -> AnyResult<RecordId> {
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
