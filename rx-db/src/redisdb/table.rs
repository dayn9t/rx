use anyhow::anyhow;
use redis::{Commands, Connection};
use std::cell::RefCell;
use std::marker::PhantomData;

use crate::redisdb::table_meta_key;
use crate::{IRecord, ITable, ITableDyn, RecordId, TableMeta};
use rx_core::text::*;

/// Redis依附变量
pub struct RedisVar<T> {
    pub name: String,
    _p: PhantomData<T>,
}
impl<T: Default + Clone + Serialize + DeserializeOwned> RedisVar<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _p: Default::default(),
        }
    }
    pub fn get(&self, conn: &RefCell<Connection>) -> BoxResult<T> {
        let s: Option<String> = conn.borrow_mut().get(&self.name)?;
        if let Some(s) = s {
            let v: T = json::from_str(&s).unwrap();
            Ok(v)
        } else {
            Ok(Default::default())
        }
    }
    fn set(&self, conn: &RefCell<Connection>, record: &T) -> BoxResult<()> {
        let s = json::to_pretty(record).unwrap();
        Ok(conn.borrow_mut().set(&self.name, &s)?)
    }
}

pub struct RedisTable<T> {
    name: String,
    meta: RedisVar<TableMeta>,
    conn: RefCell<Connection>,
    _p: PhantomData<T>,
}

impl<T> RedisTable<T> {
    pub fn new(name: String, conn: Connection) -> Self {
        let meta = RedisVar::new(table_meta_key(&name));
        Self {
            name,
            meta,
            conn: RefCell::new(conn),
            _p: PhantomData::<T>,
        }
    }
}

impl<T: IRecord> ITableDyn<T> for RedisTable<T> {
    fn open(db_url: &str, name: &str) -> BoxResult<Self>
    where
        Self: Sized,
    {
        let client = redis::Client::open(db_url)?;
        let conn = client.get_connection()?;

        Ok(Self::new(name.to_owned(), conn))
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn len(&self) -> usize {
        self.conn.borrow_mut().hlen(&self.name).unwrap()
    }

    fn get_meta(&self) -> BoxResult<TableMeta> {
        self.meta.get(&self.conn)
    }

    fn set_meta(&mut self, meta: &TableMeta) -> BoxResult<()> {
        self.meta.set(&self.conn, meta)
    }

    fn contains(&self, id: RecordId) -> bool {
        self.conn.borrow_mut().hexists(&self.name, id).unwrap()
    }

    fn get(&self, id: RecordId) -> BoxResult<T> {
        let s: Option<String> = self.conn.borrow_mut().hget(&self.name, id)?;
        if let Some(s) = s {
            let v: T = json::from_str(&s)?;
            Ok(v)
        } else {
            Err(anyhow!("Record not found"))
        }
    }

    fn put(&mut self, id: RecordId, record: &mut T) -> BoxResult<()> {
        record.set_id(id);
        let s = json::to_pretty(record).unwrap();
        self.update_last_id(id)?;
        Ok(self.conn.borrow_mut().hset(&self.name, id, &s)?)
    }

    fn delete(&mut self, id: RecordId) -> BoxResult<()> {
        Ok(self.conn.borrow_mut().hdel(&self.name, id)?)
    }

    /// 查询记录集
    fn find_all(&self) -> BoxResult<Vec<T>> {
        self.find(RecordId::default(), usize::MAX, |_| true)
    }

    /// 查询K/V对
    fn find_all_pairs(&self) -> BoxResult<Vec<(RecordId, T)>> {
        self.find_pairs(RecordId::default(), usize::max_value(), |_| true)
    }

    fn find_ids(&self, min_id: RecordId) -> BoxResult<Vec<RecordId>> {
        let ids: Vec<RecordId> = self.conn.borrow_mut().hkeys(&self.name)?;
        let mut ids: Vec<_> = ids.into_iter().filter(|id| *id >= min_id).collect();
        ids.sort();
        Ok(ids)
    }
}
impl<T: IRecord> ITable<T> for RedisTable<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::tests::*;

    #[test]
    fn tab_works() {
        let url = "redis://127.0.0.1/";
        let name = "student";

        test_table::<RedisTable<Student>>(url, name);
    }
}
