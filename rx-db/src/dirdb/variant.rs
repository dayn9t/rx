use crate::dirdb::{db_path, variant_path};
use crate::IVariant;
use rx_core::sys::fs;
use rx_core::text::*;
use std::path::PathBuf;

pub struct DirVariant<T> {
    name: String,
    path: PathBuf,
    default_value: T,
}

impl<T: Default> DirVariant<T> {
    pub fn open_path_with_default(db_path: &Path, name: &str, default: T) -> AnyResult<Self> {
        let path = variant_path(db_path, name);
        fs::make_parent(&path)?;
        Ok(DirVariant::<T> {
            name: name.to_owned(),
            path,
            default_value: default,
        })
    }
    pub fn open_path(db_path: &Path, name: &str) -> AnyResult<Self> {
        Self::open_path_with_default(db_path, name, Default::default())
    }
}

impl<T: Default + Clone + Serialize + DeserializeOwned> IVariant<T> for DirVariant<T> {
    fn open_with_default(db_url: &str, name: &str, default_value: T) -> AnyResult<Self>
    where
        Self: Sized,
    {
        let path = db_path(db_url)?;
        Self::open_path_with_default(&path, name, default_value)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.path.exists()
    }

    fn get_default(&self) -> &T {
        &self.default_value
    }

    fn get(&self) -> AnyResult<T> {
        if !self.exist() {
            Ok(self.get_default().clone())
        } else {
            json::load(&self.path)
        }
    }

    fn set(&mut self, record: &T) -> AnyResult<()> {
        json::save(&record, &self.path)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn var_works() {
        let db_url = "jddb:///tmp/jddb-test";
        let name = "var2";
        test_var::<DirVariant<Student>>(db_url, name);
    }
}
