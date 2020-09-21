use serde_derive::Serialize;

use crate::algo;

/// 获取前N元素的定长K/V对。K/V分开排列，有更好的搜索性能
#[derive(Default, Serialize)]
pub struct TopMultiMap<K, V> {
    capacity: usize,
    keys: Vec<K>,
    values: Vec<V>,
}

/// 迭代器
pub struct TopMultiMapIter<'a, K: 'a, V: 'a> {
    k_iter: std::slice::Iter<'a, K>,
    v_iter: std::slice::Iter<'a, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for TopMultiMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.k_iter.next()?, self.v_iter.next()?))
    }
}

impl<K: Ord + Clone, V> TopMultiMap<K, V> {
    /// 指定容量
    pub fn with_capacity(capacity: usize) -> TopMultiMap<K, V> {
        TopMultiMap {
            capacity,
            keys: Vec::with_capacity(capacity + 1),
            values: Vec::with_capacity(capacity + 1),
        }
    }

    /// 获取尺寸
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// 获取容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 插入元素，保证顺序，超出容量的元素被丢弃，尽量按顺序添加元素将减少元素移动
    pub fn insert(&mut self, k: K, v: V) {
        let pos = algo::upper_bound(&self.keys[..], &k);
        self.keys.insert(pos, k);
        self.values.insert(pos, v);
        if self.keys.len() > self.capacity {
            self.keys.pop();
            self.values.pop();
        }
    }

    /*
    /// 合并
    pub fn merge(&mut self, other: &Self) {
        for (k, v) in other.iter().cloned() {
            self.insert(k, v);
        }
    }*/

    /// 获取迭代器
    pub fn iter(&self) -> TopMultiMapIter<K, V> {
        TopMultiMapIter {
            k_iter: self.keys.iter(),
            v_iter: self.values.iter(),
        }
    }
    /*
        /// 获取迭代器
        pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
            self.keys.iter_mut()
        }
    */
    /// 获取第一个元素
    pub fn first(&self) -> Option<(&K, &V)> {
        Some((self.keys.first()?, self.values.first()?))
    }

    /// 获取最后一个元素
    pub fn last(&self) -> Option<(&K, &V)> {
        Some((self.keys.last()?, self.values.last()?))
    }

    /// 获取元素
    pub fn get(&self, index: usize) -> Option<(&K, &V)> {
        Some((self.keys.get(index)?, self.values.get(index)?))
    }
}

/*
impl<T> Into<Vec<T>> for TopMultiMap<K, V> {
    fn into(self) -> Vec<T> {
        self.keys
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    //use std::collections::BTreeMap;

    #[test]
    fn ff() {
        /*
        let mut m = BTreeMap::new();
        m.insert(0, 0);
        let mut it = m.iter();
        let n = it.next();
        */
    }

    #[test]
    fn top_muli_map_test() {
        let v = vec![
            18, 13, 21, 25, 11, 30, 23, 28, 15, 20, 22, 27, 12, 19, 26, 17, 29, 16, 24, 14,
        ];

        let mut top = TopMultiMap::with_capacity(5);
        for elem in &v {
            top.insert(*elem, *elem);
        }

        assert_eq!(top.first(), Some((&11, &11)));
        assert_eq!(top.get(1), Some((&12, &12)));
        assert_eq!(top.get(2), Some((&13, &13)));
        assert_eq!(top.get(3), Some((&14, &14)));
        assert_eq!(top.last(), Some((&15, &15)));

        let v5 = vec![11, 12, 13, 14, 15];
        let mut iter = v5.iter();
        for (k, v) in top.iter() {
            let a = iter.next().unwrap();
            assert_eq!(k, v);
            assert_eq!(a, v);
        }

        use std::cmp::Reverse;

        let mut top = TopMultiMap::with_capacity(5);
        for elem in &v {
            top.insert(Reverse(*elem), *elem);
        }

        assert_eq!(top.first(), Some((&Reverse(30), &30)));
        assert_eq!(top.get(1), Some((&Reverse(29), &29)));
        assert_eq!(top.get(2), Some((&Reverse(28), &28)));
        assert_eq!(top.get(3), Some((&Reverse(27), &27)));
        assert_eq!(top.last(), Some((&Reverse(26), &26)));
    }
}
