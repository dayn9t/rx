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
