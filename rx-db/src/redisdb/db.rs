use crate::{IDatabase, IRecord, ITableDyn, IVariant, RecordId};
use rx_core::text::*;

use redis::Commands;

pub const SCHEME: &str = "redis";

pub struct RedisDb {
    client: redis::Client,
}

impl IDatabase for RedisDb {
    fn open(db_url: &str) -> BoxResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn remove_variant(&self, variant_name: &str) -> BoxResult<()> {
        todo!()
    }

    fn open_variant_with_default<T>(
        &self,
        variant_name: &str,
        default: T,
    ) -> BoxResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize,
    {
        todo!()
    }

    fn remove_table(&self, table_name: &str) -> BoxResult<()> {
        todo!()
    }

    fn open_table<R: IRecord>(&self, table_name: &str) -> BoxResult<Box<dyn ITableDyn<R>>> {
        todo!()
    }

    fn find_records<R, P>(
        &self,
        table_name: &str,
        min_id: RecordId,
        limit: usize,
        predicate: P,
    ) -> BoxResult<Vec<R>>
    where
        R: IRecord,
        P: Fn(&R) -> bool,
    {
        todo!()
    }
}

impl RedisDb {
    //// 打开数据库
    //pub fn open(url: &str) -> BoxResult<Self> {
    //    let client = redis::Client::open(url)?;
    //    Ok(RedisDb { client })
    //}

    /// 打开数据库变量
    fn open_variant<T, S>(&mut self, name: S) -> BoxResult<RedisVariant<T>>
    where
        T: Default + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let conn = self.client.get_connection()?;
        RedisVariant::open(conn, name)
    }

    /// 加载数据库变量
    fn load_variant<T, S>(&mut self, name: S) -> BoxResult<T>
    where
        T: Default + Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let v: RedisVariant<T> = self.open_variant(name)?;
        v.get()
    }

    /// 打开数据库表
    fn open_table<T, S>(&mut self, name: S) -> BoxResult<RedisTable<T>>
    where
        T: Clone + DeserializeOwned + Serialize,
        S: AsRef<str>,
    {
        let conn = self.client.get_connection()?;
        //Ok(RedisTable::open(conn, name))
        todo!("open_table")
    }

    /// 删除数据库表/变量
    fn remove<S>(&self, name: S) -> BoxResult<()>
    where
        S: AsRef<str>,
    {
        let mut conn = self.client.get_connection()?;
        Ok(conn.del(name.as_ref())?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn db_works() {
        //let url = "redis://:howell.net.cn@127.0.0.1/";
        //let _db = RedisDb::open(url).unwrap();
    }
}
