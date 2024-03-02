use core::ops::Range;
use num::Integer;
use std::cmp::{max, min};
use std::collections::VecDeque;

/// 在有序集合上查找元素第一次出现的索引
pub fn search_first<T: Ord>(array: &[T], elem: &T) -> Result<usize, usize> {
    let mut index = array.binary_search(elem)?;
    for e in array[0..index].iter().rev() {
        if e == elem {
            index -= 1;
        }
    }
    Ok(index)
}

/// 在有序集合上查找元素最后一次出现的索引
pub fn search_last<T: Ord>(array: &[T], elem: &T) -> Result<usize, usize> {
    let mut index = array.binary_search(elem)?;
    for e in array[(index + 1)..].iter() {
        if e == elem {
            index += 1;
        }
    }
    Ok(index)
}

/// 在有序集合上查找元素下界
pub fn low_bound<T: Ord>(array: &[T], elem: &T) -> usize {
    *unwrap(&search_first(array, elem))
}

/// 在有序集合上查找元素上界
pub fn upper_bound<T: Ord>(array: &[T], elem: &T) -> usize {
    match search_last(array, elem) {
        Ok(i) => i + 1,
        Err(i) => i,
    }
}

/// 解包结果/错误
pub fn unwrap(r: &Result<usize, usize>) -> &usize {
    match r {
        Ok(s) => s,
        Err(s) => s,
    }
}

/// 定位包含值范围的区间段
pub fn locate_range<T: Ord>(array: &[T], value_range: &Range<T>) -> Range<usize> {
    Range {
        start: *unwrap(&search_first(array, &value_range.start)),
        end: *unwrap(&search_first(array, &value_range.end)),
    }
}

/// 可按照索引删除
trait _RemoveByIndex {
    fn remove(&mut self, index: usize);
}

/// 删除第一个满足条件的元素
pub fn deque_remove_first<T, P>(que: &mut VecDeque<T>, predicate: P) -> bool
where
    P: FnMut(&T) -> bool,
{
    if let Some(i) = que.iter().position(predicate) {
        que.remove(i);
        true
    } else {
        false
    }
}

/// 删除前N个元素
pub fn deque_pop_front_n<T>(que: &mut VecDeque<T>, n: usize) {
    for _ in 0..n {
        que.pop_front();
    }
}

/// 更新范围
pub fn update_range<T: Integer + Copy>(range: &mut Range<T>, value: T) {
    range.start = min(range.start, value);
    range.end = max(range.end, value + T::one());
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_test() {
        use super::search_first as first;

        let v = vec![0, 1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7];
        assert_eq!(first(&v[..], &-1), Err(0));
        assert_eq!(first(&v[..], &100), Err(v.len()));
        assert_eq!(first(&v[..], &0), Ok(0));
        assert_eq!(first(&v[..], &5), Ok(5));
        assert_eq!(first(&v[..], &6), Ok(11));
        //assert_eq!(v.binary_search(&6), Ok(16));

        use super::search_last as last;

        assert_eq!(last(&v[..], &-1), Err(0));
        assert_eq!(last(&v[..], &100), Err(v.len()));
        assert_eq!(last(&v[..], &0), Ok(0));
        assert_eq!(last(&v[..], &5), Ok(10));
        assert_eq!(last(&v[..], &6), Ok(17));
    }

    #[test]
    fn bound_test() {
        use super::low_bound as low;

        let v = vec![0, 1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7];
        assert_eq!(low(&v[..], &-1), 0);
        assert_eq!(low(&v[..], &100), v.len());
        assert_eq!(low(&v[..], &0), 0);
        assert_eq!(low(&v[..], &5), 5);
        assert_eq!(low(&v[..], &6), 11);

        use super::upper_bound as upper;

        assert_eq!(upper(&v[..], &-1), 0);
        assert_eq!(upper(&v[..], &100), v.len());
        assert_eq!(upper(&v[..], &0), 1);
        assert_eq!(upper(&v[..], &5), 11);
        assert_eq!(upper(&v[..], &6), 18);
    }

    #[test]
    fn locate_range_test() {
        use super::locate_range as locate;
        let v = vec![0, 1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7];
        assert_eq!(locate(&v[..], &(-1..9)), 0..20);
        assert_eq!(locate(&v[..], &(5..5)), 5..5);
        assert_eq!(locate(&v[..], &(5..6)), 5..11);
        assert_eq!(locate(&v[..], &(6..9)), 11..20);
    }

    #[test]
    fn deque_test() {
        let mut q = super::VecDeque::new();

        for i in 0..10 {
            q.push_back(i);
        }
        assert_eq!(q.len(), 10);
        assert_eq!(q.back(), Some(&9));

        if let Some(i) = q.iter().position(|x| x == &5) {
            assert_eq!(i, 5);

            super::deque_pop_front_n(&mut q, i + 1);
            assert_eq!(q.len(), 4);
        }
    }
}
