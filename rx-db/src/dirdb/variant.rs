use std::marker::PhantomData;
use std::path::PathBuf;

use rx::text::*;

use super::dirdb::DirDb;
use crate::interface::*;

pub struct DirVarient<T> {
    name: String,
    path: PathBuf,
    _p: PhantomData<T>,
}

impl<T> DirVarient<T> {
    /// 打开变量
    pub fn open<S>(db: &DirDb, name: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        Ok(DirVarient::<T> {
            name: name.as_ref().to_string(),
            path: db.varient_path(name),
            _p: PhantomData::<T>,
        })
    }
}

impl<T: Default + Serialize + DeserializeOwned> Variant for DirVarient<T> {
    type Record = T;

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.path.exists()
    }

    fn get(&self) -> Result<Self::Record> {
        load_json(&self.path)
    }

    fn set(&mut self, record: &Self::Record) -> Result<()> {
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
        let name = "student";
        db.remove_varient(name).unwrap();
        let mut var = DirVarient::open(&db, name).unwrap();

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
