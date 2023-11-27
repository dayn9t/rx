use std::path::PathBuf;

use rx_core::text::*;

use crate::interface::*;

use super::db::DirDb;

pub struct DirVariant<T> {
    name: String,
    path: PathBuf,
    default_value: Option<T>,
}

impl<T> DirVariant<T> {
    /// 打开变量
    pub fn open<S>(db: &DirDb, name: S, default_value: Option<T>) -> BoxResult<Self>
    where
        S: AsRef<str>,
    {
        Ok(DirVariant::<T> {
            name: name.as_ref().to_string(),
            path: db.variant_path(name),
            default_value,
        })
    }
}

impl<T: Default + Clone + Serialize + DeserializeOwned> IVariant for DirVariant<T> {
    type Record = T;

    type Err = Box<dyn std::error::Error>;

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.path.exists()
    }

    fn get(&self) -> BoxResult<Self::Record> {
        if !self.exist() && self.default_value.is_some() {
            Ok(self.default_value.as_ref().unwrap().clone()) // TODO: 处理default空
        } else {
            json::load(&self.path)
        }
    }

    fn set(&mut self, record: &Self::Record) -> BoxResult<()> {
        json::save(&record, &self.path)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn var_works() {
        let db = DirDb::open(&"/tmp/test/dirdb1").unwrap();
        let name = "var";
        db.remove_variant(name).unwrap();
        let mut var = DirVariant::open(&db, name, None).unwrap();

        let s1 = { Student::new(1, "Jack") };
        let s2 = { Student::new(2, "John") };
        let _s3 = { Student::new(3, "Joel") };

        assert_eq!(var.get_or_default(), Student::default());

        var.set(&s1).unwrap();
        assert_eq!(var.get().unwrap(), s1);

        var.set(&s2).unwrap();
        assert_eq!(var.get().unwrap(), s2);
    }
}
