use std::collections::BTreeMap;

use itertools::Itertools;

/// 计算直方图, 根据给定值
pub fn calc_hist<T: Ord + Clone>(values: &[T]) -> BTreeMap<T, usize> {
    let mut hist = BTreeMap::new();
    for v in values {
        *hist.entry(v.clone()).or_insert(0) += 1;
    }
    hist
}

/// 获取直方图中最多的n个元素, 降序排列
pub fn top_n<T: Ord + Clone>(hist: &BTreeMap<T, usize>, n: usize) -> Vec<(T, usize)> {
    hist.iter()
        .sorted_by_key(|&(_, v)| -(*v as i64))
        .take(n)
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/// 获取直方图中最多的前n个, 根据给定值
pub fn hist_top_n<T: Ord + Clone>(values: &[T], n: usize) -> Vec<(T, usize)> {
    top_n(&calc_hist(values), n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_hist() {
        let values = vec![1, 1, 1, 2, 3, 3, 0, 0, 0, 0];

        let hist1 = calc_hist(&values);
        let hist2 = BTreeMap::from([(0, 4), (1, 3), (2, 1), (3, 2)]);
        assert_eq!(hist1, hist2);

        let v1 = top_n(&hist1, 3);
        let v2 = vec![(0, 4), (1, 3), (3, 2)];
        assert_eq!(v1, v2);

        //println!("hist: {:?}", hist1);
        //println!("top3: {:?}", v1);
    }
}
