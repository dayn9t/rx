use std::marker::PhantomData;

use redis::Commands;

use rx::text::*;

use crate::interface::*;

use super::db::*;

//use std::collections::HashSet;

//#[derive(Size)]
pub struct RedisTable<T> {
    name: String,
    conn: redis::Connection,
    _p: PhantomData<T>,
}

impl<T> RedisTable<T> {
    /// 打开表
    pub fn open<S>(conn: redis::Connection, name: S) -> Self
    where
        S: AsRef<str>,
    {
        let name = name.as_ref().to_string();
        RedisTable::<T> {
            name,
            conn,
            _p: PhantomData::<T>,
        }
    }

    /*
    /// 记录文件全路径
    fn record_path(&self, id: usize) -> PathBuf {
        self.path.join(format!("{}.json", id))
    }*/
}

impl<T: Default + Serialize + DeserializeOwned> Table for RedisTable<T> {
    type Record = T;

    type Id = usize;

    type Err = redis::RedisError;

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&mut self) -> usize {
        self.conn.hlen(&self.name).unwrap()
    }

    fn exist(&mut self, id: Self::Id) -> bool {
        self.conn.hexists(&self.name, id).unwrap()
    }

    fn get(&mut self, id: Self::Id) -> RedisResult<Self::Record> {
        let s: String = self.conn.hget(&self.name, id)?;
        let v: Self::Record = from_str(&s).unwrap();
        Ok(v)
    }

    fn post(&mut self, record: &Self::Record) -> RedisResult<Self::Id> {
        let id = self.next_id()?;
        self.put(id, record)?;
        Ok(id)
    }

    fn put(&mut self, id: Self::Id, record: &Self::Record) -> RedisResult<()> {
        let s = to_json(record).unwrap();
        self.conn.hset(&self.name, id, &s)?;
        Ok(())
    }

    fn delete(&mut self, id: Self::Id) -> RedisResult<()> {
        self.conn.hdel(&self.name, id)?;
        Ok(())
    }

    fn find(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> RedisResult<Vec<Self::Record>> {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id)?;
        for id in ids {
            let r = self.get(id)?;
            if filter(&r) {
                vec.push(r);
                if vec.len() >= limit {
                    break;
                }
            }
        }
        Ok(vec)
    }

    fn find_pairs(
        &mut self,
        min_id: Self::Id,
        limit: usize,
        filter: &dyn Fn(&Self::Record) -> bool,
    ) -> RedisResult<Vec<(Self::Id, Self::Record)>> {
        let mut vec = Vec::new();
        let ids = self.find_ids(min_id)?;
        for id in ids {
            let r = self.get(id)?;
            if filter(&r) {
                vec.push((id, r));
                if vec.len() >= limit {
                    break;
                }
            }
        }
        Ok(vec)
    }

    fn find_ids(&mut self, min_id: Self::Id) -> RedisResult<Vec<Self::Id>> {
        let all_ids: Vec<Self::Id> = self.conn.hkeys(&self.name)?;
        let mut ids = Vec::new();
        for id in all_ids {
            if id >= min_id {
                ids.push(id);
            }
        }
        ids.sort();
        Ok(ids)
    }

    fn next_id(&mut self) -> RedisResult<Self::Id> {
        let ids = self.find_ids(0)?;
        let next = ids.last().unwrap_or(&0) + 1;
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::tests::*;

    use super::*;

    #[test]
    fn tab_works() {
        let url = "redis://127.0.0.1/";
        let name = "student";
        let mut db = RedisDb::open(url).unwrap();
        db.remove(name).unwrap();

        let mut tab = db.open_table(name).unwrap();
        assert_eq!(tab.find_ids(0).unwrap().is_empty(), true);

        let s1 = { Student::new(1, "Jack") };
        let s2 = { Student::new(2, "John") };
        let s3 = { Student::new(3, "Joel") };

        let id1 = tab.post(&s1).unwrap();
        assert_eq!(tab.get(id1).unwrap(), s1);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1]);

        let id2 = tab.post(&s2).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s2);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        tab.put(id2, &s3).unwrap();
        assert_eq!(tab.get(id2).unwrap(), s3);
        assert_eq!(tab.find_ids(0).unwrap(), vec![id1, id2]);

        let all = tab.find_all().unwrap();
        assert_eq!(all, vec![s1.clone(), s3.clone()]);

        let v = tab.find(2, 1, &|_| true).unwrap();
        assert_eq!(v, vec![s3.clone()]);

        let name = s1.name.clone();
        let v = tab.find(0, 1, &|s| s.name == name).unwrap();
        assert_eq!(v, vec![s1.clone()]);
    }
}
