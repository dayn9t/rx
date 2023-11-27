/*
pub fn group<I:Iterator<(K, V)>>(iter:I) {

}*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn search_first_test() {
        let v = vec![(1, 1), (1, 2), (2, 2), (2, 1)];
        let mut map = HashMap::new();
        let _i = v.iter();
        for (k, v) in v.iter() {
            let e = map.entry(*k).or_insert(Vec::new());
            e.push(*v);
        }
        //assert_eq!(v.binary_search(&6), Ok(16));
    }
}
