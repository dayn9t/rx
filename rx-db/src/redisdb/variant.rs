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
    fn open_with_default(db_url: &str, name: &str, default_value: T) -> AnyResult<Self>
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

    fn get(&self) -> AnyResult<T> {
        let s: Option<String> = self.conn.borrow_mut().get(&self.name)?;
        if let Some(s) = s {
            let v: T = json::from_str(&s).unwrap();
            Ok(v)
        } else {
            Ok(self.get_default().clone())
        }
    }

    fn set(&mut self, record: &T) -> AnyResult<()> {
        let s = json::to_pretty(record).unwrap();
        Ok(self.conn.borrow_mut().set(&self.name, &s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::tests::*;

    #[test]
    fn var_works() {
        let db_url = "redis://127.0.0.1/";
        let name = "student";

        test_var::<RedisVariant<Student>>(db_url, name);
    }
}
