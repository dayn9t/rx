/// 获取new中与old中不同元素的下标集合
pub fn diff<T: Eq>(new: &[T], old: &[T]) -> Vec<usize> {
    let mut vec = Vec::new();
    let len = new.len().min(old.len());
    for i in 0..len {
        let a = new.get(i).unwrap();
        let b = old.get(i).unwrap();
        if a != b {
            vec.push(i)
        };
    }

    for i in len..new.len() {
        vec.push(i);
    }
    vec
}

/// 截取制定半径内的数据(避免越界)
pub fn slice_by_radius<T: Eq>(slice: &[T], center: usize, radius: usize) -> (usize, &[T]) {
    let start = if center < radius { 0 } else { center - radius };
    let b = std::cmp::min(center + radius + 1, slice.len());
    (start, &slice[start..b])
}

/// 在切片内搜索
pub fn search_slice(slice: &[u8], value: u8) -> Vec<usize> {
    let mut a = Vec::new();
    for (i, v) in slice.iter().enumerate() {
        if *v == value {
            a.push(i);
        }
    }
    return a;
}

#[test]
fn slice_by_radius_test() {
    let v = vec![0, 1, 2, 3, 4];

    let (_start, s1) = slice_by_radius(&v[..], 1, 2);
    assert_eq!(s1.len(), 4);
    assert_eq!(s1[0], 0);
    assert_eq!(s1[3], 3);

    let (_start, s1) = slice_by_radius(&v[..], 3, 2);
    assert_eq!(s1.len(), 4);
    assert_eq!(s1[0], 1);
    assert_eq!(s1[3], 4);
}
