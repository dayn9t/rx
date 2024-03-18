/// Vec类型转换
pub fn vec_into<T1: Into<T2>, T2>(v: Vec<T1>) -> Vec<T2> {
    v.into_iter().map(|p| p.into()).collect()
}

#[cfg(test)]
mod tests {
    use crate::algo::vec_into;

    #[test]
    fn it_works() {
        let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
        let v2: Vec<i64> = vec![1, 2, 3, 4, 5];
        let v3: Vec<i64> = vec_into(v1);
        assert_eq!(v2, v3);
    }
}
