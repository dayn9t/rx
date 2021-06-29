use std::collections::{BTreeMap as Map, BTreeSet};
use std::ops::Range;
pub use std::ops::{Deref, DerefMut};

use crate::time::StopWatch;

/// ID记录数组内索引
pub type IdIndex = u32;

/// ID记录
pub trait IdRecord: PartialEq + Sized {
    /// ID类型
    type Id: Copy + Ord;

    /// 获取ID
    fn get_id(&self) -> Self::Id;

    /// 更新记录，返回是否有改动
    fn update(&mut self, record: Self) -> bool {
        if *self == record {
            false
        } else {
            *self = record;
            true
        }
    }
}

/// ID数据档案
pub trait IdInArchive<R: IdRecord> {
    /// 插入记录
    fn query<F>(&self, mut fun: F)
    where
        F: FnMut(R);
}

/// ID输出档案
pub trait IdOutArchive<R: IdRecord> {
    /// 插入记录
    fn insert(&mut self, i: IdIndex, record: &R);

    /// 更新记录
    fn update(&mut self, i: IdIndex, record: &R);
}

/// ID搜索表
#[derive(Default)]
pub struct IdMap<R: IdRecord> {
    // 人员条目
    records: Vec<R>,

    // ID查找表
    id_map: Map<R::Id, IdIndex>,

    // 插入记录操作集合
    op_insert: BTreeSet<IdIndex>,

    // 更新记录操作集合
    op_update: BTreeSet<IdIndex>,
}

impl<R: IdRecord> IdMap<R> {
    /// 判定表是否为空
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 获取表长度
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// 获取指定索引的ID
    pub fn id_at(&self, idx: IdIndex) -> R::Id {
        self.records.get(idx as usize).unwrap().get_id()
    }

    /// 获取ID对应的索引
    pub fn index_of(&self, id: R::Id) -> Option<IdIndex> {
        self.id_map.get(&id).cloned()
    }

    /// 获取ID对应记录
    pub fn get(&self, id: R::Id) -> Option<&R> {
        if let Some(idx) = self.index_of(id) {
            self.records.get(idx as usize)
        } else {
            None
        }
    }

    /// 获取第一个记录
    pub fn first(&self) -> Option<&R> {
        self.records.first()
    }

    /// 获取最后记录
    pub fn last(&self) -> Option<&R> {
        self.records.last()
    }

    /// 查找索引位置记录
    pub fn at(&self, index: IdIndex) -> Option<&R> {
        self.records.get(index as usize)
    }

    /// 查找满足条件记录
    pub fn records(&self, limit: usize) -> Vec<&R> {
        self.filter(|_| true, limit)
    }

    /// 获取记录迭代器
    pub fn iter(&self) -> std::slice::Iter<R> {
        self.records.iter()
    }

    /// 查找满足条件记录
    pub fn extract<T, F>(&self, f: F) -> Vec<T>
    where
        F: Fn(&R) -> T,
    {
        let mut v = Vec::with_capacity(self.records.len());
        for r in &self.records {
            v.push(f(r))
        }
        v
    }

    /// 查找满足条件记录
    pub fn filter<P>(&self, predicate: P, limit: usize) -> Vec<&R>
    where
        P: Fn(&R) -> bool,
    {
        let capacity = limit.min(1024);
        let mut vec = Vec::with_capacity(capacity);

        for r in &self.records {
            if !predicate(r) {
                continue;
            }
            vec.push(r);
            if vec.len() >= limit {
                break;
            }
        }
        vec
    }

    /// 查找满足条件记录索引
    pub fn filter_indexes<P>(&self, predicate: P, limit: usize) -> Vec<IdIndex>
    where
        P: Fn(&R) -> bool,
    {
        let capacity = limit.min(1024);
        let mut vec = Vec::with_capacity(capacity);

        for (i, r) in self.records.iter().enumerate() {
            if !predicate(r) {
                continue;
            }
            vec.push(i as IdIndex);
            if vec.len() >= limit {
                break;
            }
        }
        vec
    }

    /// 获取索引区间记录集合
    pub fn range(&self, range: Range<usize>, limit: usize) -> Vec<&R> {
        let mut infos = Vec::new();
        for info in &self.records[range] {
            infos.push(info);
            if infos.len() >= limit {
                break;
            }
        }
        infos
    }

    /// 追加记录，确保id不会重复
    pub fn append(&mut self, record: R) -> IdIndex {
        let idx = self.records.len() as IdIndex;
        self.id_map.insert(record.get_id(), idx);
        self.records.push(record);
        assert_eq!(self.id_map.len(), self.records.len());
        idx
    }

    /// 更新记录或者添加（如记录不能存在）
    pub fn update(&mut self, record: R) -> IdIndex {
        let id = record.get_id();
        if let Some(idx) = self.id_map.get(&id) {
            let r = self.records.get_mut(*idx as usize).unwrap();
            if r.update(record) && !self.op_insert.contains(idx) {
                // 操作不存在时才需要插入，否则都是重复，如果要支持删除操作，这里要修改
                self.op_update.insert(*idx);
            }
            *idx
        } else {
            let idx = self.append(record);
            self.op_insert.insert(idx);
            idx
        }
    }

    /// 加载记录
    pub fn load<A>(&mut self, archive: &A) -> usize
    where
        A: IdInArchive<R>,
    {
        archive.query(|r| {
            self.append(r);
        });
        self.len()
    }

    /// 保存修改
    pub fn save<A>(&mut self, archive: &mut A) -> usize
    where
        A: IdOutArchive<R>,
    {
        let len = self.op_insert.len() + self.op_update.len();

        let mut sw = StopWatch::new();

        for i in &self.op_insert {
            let r = self.at(*i).unwrap();
            sw.start();
            archive.insert(*i, r);
            sw.stop();
        }
        self.op_insert.clear();

        println!("insert: {}", sw);
        let mut sw = StopWatch::new();

        for i in &self.op_update {
            let r = self.at(*i).unwrap();
            sw.start();
            archive.update(*i, r);
            sw.stop();
        }
        println!("update: {}", sw);
        self.op_update.clear();
        len
    }
}

#[cfg(test)]
mod tests {
    #[derive(Default, PartialEq, Debug, Clone)]
    pub struct Student {
        id: i32,
        name: String,
    }

    impl super::IdRecord for Student {
        type Id = i32;

        fn get_id(&self) -> Self::Id {
            self.id
        }
    }

    type StudentTab = super::IdMap<Student>;

    #[test]
    fn test_table() {
        let mut tab = StudentTab::default();
        assert!(tab.is_empty());

        // test push & get_record
        for id in 1..11 {
            let s = Student {
                id,
                name: "jack".to_string(),
            };
            let idx = tab.append(s.clone());
            assert_eq!(idx as i32, id - 1);
            assert_eq!(tab.len(), idx as usize + 1);
            assert_eq!(tab.index_of(id).unwrap(), idx);
            assert_eq!(tab.id_at(idx), id);

            assert_eq!(tab.get(id).unwrap(), &s);
        }

        // test find_records
        let rs = tab.filter(|r| r.id % 2 == 0, 10);
        assert_eq!(rs.len(), 5);

        let s2 = Student {
            id: 2,
            name: "jack".to_string(),
        };
        assert_eq!(rs[0], &s2);

        let s10 = Student {
            id: 10,
            name: "jack".to_string(),
        };
        assert_eq!(rs[4], &s10);

        // test limit
        let rs = tab.filter(|r| r.id % 2 == 0, 1);
        assert_eq!(rs.len(), 1);

        // test update
        let s1 = Student {
            id: 1,
            name: "first".to_string(),
        };
        let s10 = Student {
            id: 10,
            name: "tenth".to_string(),
        };

        assert_eq!(tab.update(s1.clone()), 0);
        assert_eq!(tab.update(s10.clone()), 9);
        assert_eq!(tab.get(1).unwrap(), &s1);
        assert_eq!(tab.get(10).unwrap(), &s10);
    }
}
