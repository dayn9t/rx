#[cfg(test)]
pub mod tests {

    use crate::*;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Record)]
    pub struct Student {
        pub id: Option<usize>,
        pub name: String,
    }

    impl Student {
        pub fn new(id: usize, name: &str) -> Student {
            Student {
                id: Some(id),
                name: name.to_string(),
            }
        }
    }

    pub fn test_var<V: IVariant<Student>>(db_url: &str, name: &str) {
        remove_variant(db_url, name).unwrap();

        let mut var = V::open(db_url, name).unwrap();
        assert_eq!(var.name(), name);
        assert!(!var.exist());

        let s1 = { Student::new(1, "Jack") };
        let s2 = { Student::new(2, "John") };
        let _s3 = { Student::new(3, "Joel") };

        assert_eq!(var.get_or_default(), Student::default());

        var.set(&s1).unwrap();
        assert!(var.exist());
        assert_eq!(var.get().unwrap(), s1);

        var.set(&s2).unwrap();
        assert_eq!(var.get().unwrap(), s2);
    }

    pub fn test_table<T: ITable<Student>>(db_url: &str, name: &str) {
        remove_table(db_url, name).unwrap();

        let mut tab = T::open(db_url, name).unwrap();
        assert!(tab.is_empty());
        assert!(tab.find_ids(&None).unwrap().is_empty());

        let mut s1 = { Student::new(1, "Jack") };
        let mut s2 = { Student::new(2, "John") };
        let mut s3 = { Student::new(3, "Joel") };

        let id1 = tab.post(&mut s1, &None).unwrap();
        assert_eq!(tab.get(&id1, &None).unwrap(), s1);
        assert_eq!(tab.find_ids(&None).unwrap(), vec![id1]);

        let id2 = tab.post(&mut s2, &None).unwrap();
        assert_eq!(tab.get(&id2, &None).unwrap(), s2);
        assert_eq!(tab.find_ids(&None).unwrap(), vec![id1, id2]);

        tab.put(&id2, &mut s3, &None).unwrap();
        assert_eq!(tab.get(&id2, &None).unwrap(), s3);
        assert_eq!(tab.find_ids(&None).unwrap(), vec![id1, id2]);

        let all = tab.find_all(&None).unwrap();
        assert_eq!(all, vec![s1.clone(), s3.clone()]);

        let name = s1.name.clone();
        let v = tab.find(1, |s| s.name == name, &None).unwrap();
        assert_eq!(v, vec![s1.clone()]);
    }
}
