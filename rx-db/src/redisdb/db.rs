use super::table::*;
use super::variant::*;

use crate::Variant;
use rx::text::*;

use redis::Commands;
pub use redis::RedisResult;

pub struct RedisDb {
    client: redis::Client,
}

impl RedisDb {
    /// 打开数据库
    pub fn open(url: &str) -> RedisResult<Self> {
        let client = redis::Client::open(url)?;
        Ok(RedisDb { client })
    }

    /// 打开数据库变量
    pub fn open_variant<T, S>(&mut self, name: S) -> RedisResult<RedisVariant<T>>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let conn = self.client.get_connection()?;
        Ok(RedisVariant::open(conn, name))
    }

    /// 加载数据库变量
    pub fn load_variant<T, S>(&mut self, name: S) -> RedisResult<T>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let mut v: RedisVariant<T> = self.open_variant(name)?;
        v.get()
    }

    /// 打开数据库表
    pub fn open_table<T, S>(&mut self, name: S) -> RedisResult<RedisTable<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let conn = self.client.get_connection()?;
        Ok(RedisTable::open(conn, name))
    }

    /// 删除数据库表/变量
    pub fn remove<S>(&self, name: S) -> RedisResult<()>
    where
        S: AsRef<str>,
    {
        let mut conn = self.client.get_connection()?;
        conn.del(name.as_ref())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_works() {
        let url = "redis://127.0.0.1/";
        let _db = RedisDb::open(url).unwrap();
    }
}
