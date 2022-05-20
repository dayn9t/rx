use std::marker::PhantomData;

use redis::Commands;

use rx::text::*;

use crate::interface::*;

use super::db::*;

pub struct RedisVariant<T> {
    name: String,
    conn: redis::Connection,
    _p: PhantomData<T>,
}

impl<T> RedisVariant<T> {
    /// 打开变量
    pub fn open<S>(conn: redis::Connection, name: S) -> Self
    where
        S: AsRef<str>,
    {
        RedisVariant::<T> {
            name: name.as_ref().to_string(),
            conn,
            _p: PhantomData::<T>,
        }
    }
}

impl<T: Default + Serialize + DeserializeOwned> Variant for RedisVariant<T> {
    type Record = T;

    type Err = redis::RedisError;

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&mut self) -> bool {
        self.conn.exists(&self.name).unwrap()
    }

    fn get(&mut self) -> RedisResult<Self::Record> {
        let s: String = self.conn.get(&self.name)?;
        let v: Self::Record = from_str(&s).unwrap();
        Ok(v)
    }

    fn set(&mut self, record: &Self::Record) -> RedisResult<()> {
        let s = to_json(record).unwrap();
        self.conn.set(&self.name, &s)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn var_works() {
        let mut db = RedisDb::open(&"/tmp/test/dirdb1").unwrap();
        let name = "student";
        db.remove(name).unwrap();
        let mut var = db.open_variant(name).unwrap();

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
