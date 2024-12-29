use crate::{IDatabase, IRecord, ITable, ITableDyn, IVariant, RecordId};
use anyhow::anyhow;
use rx_core::text::*;

use crate::redisdb::{RedisTable, RedisVariant};
use redis::{Commands, Connection};
use url::Url;

pub const SCHEME: &str = "redis";

pub struct RedisDb {
    client: redis::Client,
}

impl IDatabase for RedisDb {
    fn open(db_url: &str) -> BoxResult<Self>
    where
        Self: Sized,
    {
        let uri = Url::parse(db_url)?;
        if uri.scheme() != SCHEME {
            return Err(anyhow!("Invalid scheme"));
        }
        let client = redis::Client::open(db_url)?;
        Ok(RedisDb { client })
    }

    fn remove_variant(&self, variant_name: &str) -> BoxResult<()> {
        self.del(variant_name)
    }

    fn open_variant_with_default<T>(
        &self,
        variant_name: &str,
        default: T,
    ) -> BoxResult<Box<dyn IVariant<T>>>
    where
        T: Default + DeserializeOwned + Serialize + Clone + 'static,
    {
        let conn = self.get_connection()?;
        let v = RedisVariant::new(conn, variant_name.to_owned(), default);
        Ok(Box::new(v))
    }

    fn remove_table(&self, table_name: &str) -> BoxResult<()> {
        self.del(table_name)
    }

    fn open_table<R: IRecord + 'static>(
        &self,
        table_name: &str,
    ) -> BoxResult<Box<dyn ITableDyn<R>>> {
        let conn = self.get_connection()?;
        let v = RedisTable::new(table_name.to_owned(), conn);
        Ok(Box::new(v))
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
        let conn = self.get_connection()?;
        let table = RedisTable::<R>::new(table_name.to_owned(), conn);
        table.find(min_id, limit, predicate)
    }
}

impl RedisDb {
    /// 获取连接
    pub fn get_connection(&self) -> BoxResult<Connection> {
        let conn = self.client.get_connection()?;
        Ok(conn)
    }

    fn del(&self, key: &str) -> BoxResult<()> {
        let mut conn = self.client.get_connection()?;
        Ok(conn.del(key)?)
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
