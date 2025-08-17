use crate::dirdb::{DirVariant, EXT, db_path, meta_path};
use crate::{HttpError, IRecord, IRecordId, ITable, ITableDyn, IVariant, TableMeta};
use path_macro::path;
use rx_core::sys::fs::{SortOrder, find_file_by_ext, find_file_by_name};
use rx_core::{sys::fs, text::*};
use std::marker::PhantomData;
use std::path::PathBuf;

//#[derive(Size)]
pub struct DirTable<R: IRecord> {
    name: String,
    path: PathBuf,
    meta: DirVariant<TableMeta<R::RecordId>>,
    _p: PhantomData<R>,
}

impl<R: IRecord> DirTable<R> {
    /// 打开表
    pub fn open_path(db_path: &Path, name: &str) -> AnyResult<Self> {
        let path = path!(db_path / name);
        fs::ensure_dir_exist(&path)?;
        let meta = DirVariant::open_path(&meta_path(db_path), &name)?;
        Ok(Self {
            name: name.to_owned(),
            path,
            meta,
            _p: PhantomData::<R>,
        })
    }

    /// 数据库表路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    fn record_name(id: &R::RecordId) -> String {
        format!("{}.{}", id, EXT)
    }

    /// 记录文件全路径
    fn record_path(&self, id: &R::RecordId, partition_id: &Option<String>) -> PathBuf {
        let name = Self::record_name(&id);
        let name = if let Some(partition_id) = partition_id {
            format!("{}/{}", partition_id, name)
        } else {
            name
        };
        path!(self.path / name)
    }

    /// 查找记录文件全路径，未指定partition_id则会递归搜索
    fn find_record_path(
        &self,
        id: &R::RecordId,
        partition_id: &Option<String>,
    ) -> AnyResult<PathBuf> {
        let name = Self::record_name(id);
        let record_dir = if let Some(partition_id) = partition_id {
            path!(self.path / partition_id.to_string())
        } else {
            self.path.clone()
        };
        let files = find_file_by_name(&record_dir, &name).unwrap_or_default();
        if files.is_empty() {
            Err(HttpError::not_found(format!("record not found: {}", id)).into())
        } else if files.len() > 1 {
            Err(HttpError::internal_server_error(format!(
                "duplicate record: {} => {:?}",
                id, files
            ))
            .into())
        } else {
            Ok(files[0].clone())
        }
    }
}

impl<R: IRecord> ITableDyn<R> for DirTable<R> {
    fn open(db_url: &str, name: &str) -> AnyResult<Self> {
        let db_path = db_path(db_url)?;
        Self::open_path(&db_path, name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_meta(&self) -> AnyResult<TableMeta<R::RecordId>> {
        self.meta.get()
    }

    fn set_meta(&mut self, meta: &TableMeta<R::RecordId>) -> AnyResult<()> {
        self.meta.set(meta)
    }

    fn contains(&self, id: &R::RecordId) -> bool {
        self.find_record_path(id, &None).is_ok()
    }

    fn get(&self, id: &R::RecordId, partition_id: &Option<String>) -> AnyResult<R> {
        let p = self.find_record_path(id, partition_id)?;
        json::load(&p)
    }

    fn put(
        &mut self,
        id: &R::RecordId,
        record: &mut R,
        partition_id: &Option<String>,
    ) -> AnyResult<()> {
        record.set_id(id);
        let p = match self.find_record_path(id, partition_id) {
            Ok(p) => p,
            Err(_) => self.record_path(id, partition_id),
        };
        // FIXME: 路径变化，删除老文件
        json::save(&record, &p)?;
        self.update_last_id(id)
    }

    fn delete(&mut self, id: &R::RecordId, partition_id: &Option<String>) -> AnyResult<()> {
        let p = self.find_record_path(id, partition_id)?; // FIXME：删除不存在的会报错
        Ok(fs::remove(&p)?)
    }

    /// 查询记录集
    fn find_all(&self, partition_id: &Option<String>) -> AnyResult<Vec<R>> {
        self.find(|_| true, partition_id)
    }

    /// 查询K/V对
    fn find_all_pairs(&self, partition_id: &Option<String>) -> AnyResult<Vec<(R::RecordId, R)>> {
        self.find_pairs(|_| true, partition_id)
    }

    fn find_ids(&self, partition_id: &Option<String>) -> AnyResult<Vec<R::RecordId>> {
        find_record_ids(&self.path, partition_id)
    }
}

impl<R: IRecord> ITable<R> for DirTable<R> {
    /// 查询记录集
    fn find<P>(&self, predicate: P, partition_id: &Option<String>) -> AnyResult<Vec<R>>
    where
        P: Fn(&R) -> bool,
    {
        let dir = if let Some(partition_id) = partition_id {
            path!(self.path / partition_id.to_string())
        } else {
            self.path.clone()
        };
        let files = find_file_by_ext(dir, EXT)?;

        let mut records = Vec::new();
        for file in files {
            let r = json::load(&file)?;
            if predicate(&r) {
                records.push(r);
            }
        }
        Ok(records)
    }
}

/// 从路径中查找记录ID
fn find_record_ids<RID: IRecordId>(
    path: &Path,
    partition_id: &Option<String>,
) -> AnyResult<Vec<RID>> {
    let mut ids = Vec::new();

    let names = if let Some(partition_id) = partition_id {
        let path = path!(path / partition_id.to_string());
        if !path.exists() {
            return Ok(ids);
        }
        fs::file_stems_in(path, EXT, SortOrder::None)?
    } else {
        fs::file_stems_in(path, EXT, SortOrder::None)?
    };

    for stem in names {
        if let Ok(id) = RID::from_str(&stem) {
            ids.push(id);
        }
    }
    ids.sort();
    Ok(ids)
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
