/// VecMap.
///
/// 用Vec实现的Map, 保证遍历元素的顺序与插入顺序一致
///
#[derive(Default)]
pub struct VecMap {
    vars: Vec<(String, String)>,
}

impl VecMap {
    /// 创建
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    /// 获取变量值
    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    /// 设置变量的值
    pub fn insert(&mut self, key: &str, value: &str) {
        match self.vars.iter_mut().find(|(k, _)| k == key) {
            Some(kv) => kv.1 = value.to_string(),
            None => self.vars.push((key.to_string(), value.to_string())),
        }
    }
}

pub struct VecMapIter<'a> {
    inner: std::slice::Iter<'a, (String, String)>,
}

impl<'a> Iterator for VecMapIter<'a> {
    type Item = (&'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, v))
    }
}

impl<'a> IntoIterator for &'a VecMap {
    type Item = (&'a String, &'a String);
    type IntoIter = VecMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        VecMapIter {
            inner: self.vars.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_map() {
        let mut map = VecMap::new();

        // Test set
        map.insert("key1", "value1");
        assert_eq!(map.get("key1"), Some(&"value1".to_string()));

        // Test update
        map.insert("key1", "value2");
        assert_eq!(map.get("key1"), Some(&"value2".to_string()));

        // Test get non-existing key
        assert_eq!(map.get("key2"), None);

        for (key, value) in &map {
            println!("{}: {}", key, value);
        }
    }
}
