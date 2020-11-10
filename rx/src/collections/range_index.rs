use core::ops::Range;

use serde_derive::Serialize;

use crate::algo;

/// 区间索引，用于在一个有序数组上，定位区间
#[derive(Default, Serialize)]
pub struct RangeIndex<T> {
    // 用于索引的元素数组
    elems: Vec<T>,
}

impl<T: Ord> RangeIndex<T> {
    /// 指定容量
    pub fn with_capacity(capacity: usize) -> RangeIndex<T> {
        RangeIndex {
            elems: Vec::with_capacity(capacity),
        }
    }

    /// 获取尺寸
    pub fn len(&self) -> usize {
        self.elems.len()
    }

    /// 判定否为空
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    /// 定位索引(下标)区间
    pub fn locate(&self, value_range: &Range<T>) -> Range<usize> {
        algo::locate_range(&self.elems[..], &value_range)
    }

    /// 追加条目，用户保证数据有序
    pub fn push(&mut self, elem: T) {
        if let Some(e) = self.elems.last() {
            assert!(elem >= *e);
        }
        self.elems.push(elem);
    }

    /// 清除数据
    pub fn clear(&mut self) {
        self.elems.clear();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_first_test() {
        //let r1 = 1..5;
        //assert_eq!(v.binary_search(&6), Ok(16));
    }
}
