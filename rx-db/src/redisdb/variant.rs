use std::cell::RefCell;
use std::marker::PhantomData;

use redis::Commands;

use crate::IVariant;
use rx_core::text::*;

pub struct RedisVariant<T> {
    name: String,
    conn: RefCell<redis::Connection>,
    _p: PhantomData<T>,
}

impl<T> RedisVariant<T> {
    /// 打开变量
    pub fn open<S>(conn: redis::Connection, name: S) -> BoxResult<Self>
    where
        S: AsRef<str>,
    {
        Ok(RedisVariant::<T> {
            name: name.as_ref().to_string(),
            conn: RefCell::new(conn),
            _p: PhantomData::<T>,
        })
    }
}

impl<T: Default + Clone + Serialize + DeserializeOwned> IVariant for RedisVariant<T> {
    type Record = T;

    fn open(db_url: &str, variant_name: &str) -> BoxResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn remove(db_url: &str, variant_name: &str) -> BoxResult<()> {
        todo!()
    }

    fn exists(db_url: &str, variant_name: &str) -> BoxResult<bool> {
        todo!()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.conn.borrow_mut().exists(&self.name).unwrap()
    }

    fn get(&self) -> BoxResult<Self::Record> {
        let s: String = self.conn.borrow_mut().get(&self.name)?;
        let v: Self::Record = json::from_str(&s).unwrap();
        Ok(v)
    }

    fn set(&mut self, record: &Self::Record) -> BoxResult<()> {
        let s = json::to_pretty(record).unwrap();
        Ok(self.conn.borrow_mut().set(&self.name, &s)?)
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::RedisDb;
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn var_works() {
        let url = "redis://:howell.net.cn@127.0.0.1/";
        let name = "v\0a\0r";

        let mut db = RedisDb::open(url).unwrap();

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
*/