use std::cell::RefCell;

use redis::{Commands, Connection};

use crate::redisdb::RedisDb;
use crate::{IDatabase, IVariant};
use rx_core::text::*;

pub struct RedisVariant<T> {
    name: String,
    conn: RefCell<Connection>,
    default_value: T,
}

impl<T: Default + Clone + Serialize + DeserializeOwned> RedisVariant<T> {
    pub fn new(conn: Connection, name: String, default_value: T) -> Self {
        Self {
            name,
            conn: RefCell::new(conn),
            default_value,
        }
    }
}

impl<T: Default + Clone + Serialize + DeserializeOwned> IVariant<T> for RedisVariant<T> {
    fn open_with_default(db_url: &str, name: &str, default_value: T) -> BoxResult<Self>
    where
        Self: Sized,
    {
        let db = RedisDb::open(db_url)?;
        let conn = db.get_connection()?;

        Ok(Self::new(conn, name.to_owned(), default_value))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn exist(&self) -> bool {
        self.conn.borrow_mut().exists(&self.name).unwrap()
    }

    fn get_default(&self) -> &T {
        &self.default_value
    }

    fn get(&self) -> BoxResult<T> {
        let s: String = self.conn.borrow_mut().get(&self.name)?;
        let v: T = json::from_str(&s).unwrap();
        Ok(v)
    }

    fn set(&mut self, record: &T) -> BoxResult<()> {
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
