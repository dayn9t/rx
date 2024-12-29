use std::cell::RefCell;
use std::marker::PhantomData;

use redis::{Commands, Connection};

use crate::{IRecord, ITable, ITableDyn, RecordId};
use rx_core::text::*;

pub struct RedisTable<T> {
    name: String,
    meta_name: String,
    conn: RefCell<Connection>,
    _p: PhantomData<T>,
}

impl<T> RedisTable<T> {
    pub fn new(conn: Connection, name: String) -> Self {
        let meta_name = format!("{}_meta", name);
        Self {
            name,
            meta_name,
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

        let name = name.to_owned();
        let meta_name = format!("{}_meta", name);
        Ok(Self {
            name,
            meta_name,
            conn: RefCell::new(conn),
            _p: PhantomData::<T>,
        })
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn len(&self) -> usize {
        self.conn.borrow_mut().hlen(&self.name).unwrap()
    }

    fn contains(&self, id: RecordId) -> bool {
        self.conn.borrow_mut().hexists(&self.name, id).unwrap()
    }

    fn get(&self, id: RecordId) -> BoxResult<T> {
        let s: String = self.conn.borrow_mut().hget(&self.name, id)?;
        let v: T = json::from_str(&s).unwrap();
        Ok(v)
    }

    fn put(&mut self, id: RecordId, record: &mut T) -> BoxResult<()> {
        record.set_id(id);
        let s = json::to_pretty(record).unwrap();
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

    fn next_id(&mut self) -> BoxResult<RecordId> {
        let ids = self.find_ids(0)?;
        let next = ids.last().unwrap_or(&0) + 1;
        Ok(next)
    }
}
impl<T: IRecord> ITable<T> for RedisTable<T> {}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::RedisDb;
    use crate::test::tests::*;

    #[test]
    fn tab_works() {
        let url = "redis://127.0.0.1/";
        let name = "student";
        let mut db = RedisDb::open(url).unwrap();
        db.remove(name).unwrap();

        let mut tab = db.open_table(name).unwrap();
        assert_eq!(tab.is_empty(), true);

        let mut s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let mut s3 = { Student::new(3, "Joel") };

        let id1 = tab.post(&mut s1).unwrap();
        assert_eq!(tab.get(id1).unwrap(), s1);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1]);

        let id2 = tab.post(&mut s2).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s2);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        tab.put(id2, &mut s3).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s3);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        let all = tab.find_all().unwrap();
        assert_eq!(all, vec![s1.clone(), s3.clone()]);

        let v = tab.find(2, 1, |_| true).unwrap();
        assert_eq!(v, vec![s3.clone()]);

        let name = s1.name.clone();
        let v = tab.find(0, 1, |s| s.name == name).unwrap();
        assert_eq!(v, vec![s1.clone()]);

        for i in 1..200 {
            let _id1 = tab.post(&mut s1).unwrap();
        }
    }
}
*/
