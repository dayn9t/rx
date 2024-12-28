use crate::IVariant;
use crate::dirdb::variant_path;
use rx_core::sys::fs;
use rx_core::text::*;
use std::path::PathBuf;

pub struct DirVariant<T> {
    name: String,
    path: PathBuf,
    default_value: T,
}

impl<T: Default + Clone + Serialize + DeserializeOwned> IVariant<T> for DirVariant<T> {
    fn open_with_default(db_url: &str, name: &str, default_value: T) -> BoxResult<Self>
    where
        Self: Sized,
    {
        let path = variant_path(db_url, name)?;
        fs::make_parent(&path)?;
        Ok(DirVariant::<T> {
            name: name.to_owned(),
            path,
            default_value,
        })
    }

    fn remove(db_url: &str, name: &str) -> BoxResult<()> {
        let path = variant_path(db_url, name)?;
        Ok(fs::remove(&path)?)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.path.exists()
    }

    fn get(&self) -> BoxResult<T> {
        if !self.exist() {
            Ok(self.default_value.clone()) // TODO: 处理default空
        } else {
            json::load(&self.path)
        }
    }

    fn set(&mut self, record: &T) -> BoxResult<()> {
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
