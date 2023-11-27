use crate::serde_export::*;

use crate::algo;

/// 获取前N元素的定长队列
#[derive(Default, Serialize)]
pub struct TopQueue<T> {
    capacity: usize,
    elems: Vec<T>,
}

impl<T: Ord + Clone> TopQueue<T> {
    /// 指定容量
    pub fn with_capacity(capacity: usize) -> TopQueue<T> {
        TopQueue {
            capacity,
            elems: Vec::with_capacity(capacity + 1),
        }
    }

    /// 获取尺寸
    pub fn len(&self) -> usize {
        self.elems.len()
    }

    /// 获取容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 插入元素，保证顺序，超出容量的元素被丢弃，尽量按顺序添加元素将减少元素移动
    pub fn insert(&mut self, elem: T) {
        let pos = algo::upper_bound(&self.elems[..], &elem);
        self.elems.insert(pos, elem);
        if self.elems.len() > self.capacity {
            self.elems.pop();
        }
    }

    /// 合并
    pub fn merge(&mut self, other: &Self) {
        for elem in other.iter().cloned() {
            self.insert(elem);
        }
    }

    /// 获取迭代器
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elems.iter()
    }

    /// 获取迭代器
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.elems.iter_mut()
    }

    /// 获取第一个元素
    pub fn first(&self) -> Option<&T> {
        self.elems.first()
    }

    /// 获取最后一个元素
    pub fn last(&self) -> Option<&T> {
        self.elems.last()
    }

    /// 获取元素
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elems.get(index)
    }
}

impl<T> Into<Vec<T>> for TopQueue<T> {
    fn into(self) -> Vec<T> {
        self.elems
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_queue_test() {
        let v = vec![
            18, 13, 21, 25, 11, 30, 23, 28, 15, 20, 22, 27, 12, 19, 26, 17, 29, 16, 24, 14,
        ];

        let mut top = TopQueue::with_capacity(5);
        for elem in &v {
            top.insert(*elem);
        }

        assert_eq!(top.first(), Some(&11));
        assert_eq!(top.get(1), Some(&12));
        assert_eq!(top.get(2), Some(&13));
        assert_eq!(top.get(3), Some(&14));
        assert_eq!(top.last(), Some(&15));

        use std::cmp::Reverse;

        let mut top = TopQueue::with_capacity(5);
        for elem in &v {
            top.insert(Reverse(*elem));
        }

        assert_eq!(top.first(), Some(&Reverse(30)));
        assert_eq!(top.get(1), Some(&Reverse(29)));
        assert_eq!(top.get(2), Some(&Reverse(28)));
        assert_eq!(top.get(3), Some(&Reverse(27)));
        assert_eq!(top.last(), Some(&Reverse(26)));
    }
}
