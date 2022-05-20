use std::marker::PhantomData;
use std::path::PathBuf;

use rx::text::*;

use super::db::DirDb;
use crate::interface::*;

type IoResult<T> = std::io::Result<T>;

pub struct DirVariant<T> {
    name: String,
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T> DirVariant<T> {
    /// 打开变量
    pub fn open<S>(db: &DirDb, name: S) -> IoResult<Self>
    where
        S: AsRef<str>,
    {
        Ok(DirVariant::<T> {
            name: name.as_ref().to_string(),
            path: db.variant_path(name),
            _p: PhantomData::<T>,
        })
    }
}

impl<T: Default + Serialize + DeserializeOwned> Variant for DirVariant<T> {
    type Record = T;

    type Err = std::io::Error;

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&mut self) -> bool {
        self.path.exists()
    }

    fn get(&mut self) -> IoResult<Self::Record> {
        load_json(&self.path)
    }

    fn set(&mut self, record: &Self::Record) -> IoResult<()> {
        save_json(&record, &self.path)
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
        let mut var = DirVariant::open(&db, name).unwrap();

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
