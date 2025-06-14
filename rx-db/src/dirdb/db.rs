use crate::dirdb::{DirTable, DirVariant};
use crate::{IDatabase, IRecord, ITable, ITableDyn, IVariant, RecordId};
use anyhow::anyhow;
use path_macro::path;
use rx_core::sys::fs;
use rx_core::text::*;
use std::path::PathBuf;
use url::Url;

pub const SCHEME: &str = "jddb";
pub const EXT: &str = "json";

/// 获取数据库对象路径，从URL和表名解析路径
pub fn db_path(db_url: &str) -> AnyResult<PathBuf> {
    let uri = Url::parse(db_url)?;
    if uri.scheme() != SCHEME {
        return Err(anyhow!("Invalid scheme"));
    }
    Ok(path!(uri.path()))
}

pub fn meta_path(db_path: &Path) -> PathBuf {
    path!(db_path / ".meta")
}

pub fn table_meta_path(db_path: &Path, table_name: &str) -> PathBuf {
    variant_path(&meta_path(db_path), table_name)
}

/// 表路径
pub fn table_path(db_path: &Path, table_name: &str) -> PathBuf {
    path!(db_path / table_name)
}

/// 变量路径
pub fn variant_path(db_path: &Path, variant_name: &str) -> PathBuf {
    path!(db_path / format!("{variant_name}.{EXT}"))
}

pub struct DirDb {
    path: PathBuf,
}

impl IDatabase for DirDb {
    fn open(db_url: &str) -> AnyResult<Self>
    where
        Self: Sized,
    {
        let path = db_path(db_url)?;
        Self::open_path(&path)
    }

    fn remove_variant(&self, variant_name: &str) -> AnyResult<()> {
        let path = variant_path(&self.path, variant_name);
        Self::remove_file(&path)
    }

    fn open_variant_with_default<T>(
        &self,
        variant_name: &str,
        default: T,
    ) -> AnyResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static,
    {
        let tab = DirVariant::<T>::open_path_with_default(&self.path, variant_name, default)?;
        Ok(Box::new(tab))
    }

    fn remove_table(&self, table_name: &str) -> AnyResult<()> {
        let path = table_path(&self.path, table_name);
        Self::remove_dir(&path)?;
        let meta_path = table_meta_path(&self.path, table_name);
        Self::remove_file(&meta_path)
    }

    fn open_table<R: IRecord + 'static>(
        &self,
        table_name: &str,
    ) -> AnyResult<Box<dyn ITableDyn<R>>> {
        let tab = DirTable::<R>::open_path(&self.path, table_name)?;
        Ok(Box::new(tab))
    }

    fn find_records<R, P>(
        &self,
        table_name: &str,
        min_id: RecordId,
        limit: usize,
        predicate: P,
        partition_id: Option<u32>,
    ) -> AnyResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool,
    {
        let table = DirTable::<R>::open_path(&self.path, table_name)?;
        table.find(min_id, limit, predicate, partition_id)
    }
}
impl DirDb {
    pub fn open_path(db_path: &Path) -> AnyResult<Self>
    where
        Self: Sized,
    {
        fs::ensure_dir_exist(&db_path)?;
        Ok(DirDb {
            path: db_path.to_path_buf(),
        })
    }

    /// 数据库名称
    pub fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    /// 数据库路径
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn remove_file(path: &Path) -> AnyResult<()> {
        if path.exists() && !path.is_file() {
            return Err(anyhow!("File path not file"));
        }
        Ok(fs::remove(&path)?)
    }

    pub fn remove_dir(path: &Path) -> AnyResult<()> {
        if path.exists() && !path.is_dir() {
            return Err(anyhow!("File path not dir"));
        }
        Ok(fs::remove(&path)?)
    }
}

/*

#[cfg(test)]
mod tests {
    use crate::db::*;
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn db_works() {
        let s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let _s3 = { Student::new(3, "Joel") };

        let mut db = DirDb::open(&"/tmp/test/dir_db1").unwrap();

        assert_eq!(db.name(), "dir_db1");

        let mut var = db.open_variant(&"var", None).unwrap();
        var.set(&s1).unwrap();

        let mut tab = db.open_table(&"student").unwrap();
        tab.put(1, &mut s2).unwrap();
    }
}
*/
