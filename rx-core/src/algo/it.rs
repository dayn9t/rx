/// 获取可迭代对象最大值和索引
pub fn max_index<T: PartialOrd + Clone>(iter: impl Iterator<Item = T>) -> Option<(usize, T)> {
    let mut max = None;
    for (i, v) in iter.enumerate() {
        if let Some((_, max_v)) = max.clone() {
            if v > max_v {
                max = Some((i, v));
            }
        } else {
            max = Some((i, v));
        }
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let arr = [1, 2, 3, 4, 5];
        let max = super::max_index(arr.iter());
        assert_eq!(max, Some((4usize, &5)));

        let arr = [1.1, 2.0, 1.3];
        let max = super::max_index(arr.iter());
        assert_eq!(max, Some((1usize, &2.0)));
    }
}
