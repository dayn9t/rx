#[cfg(test)]
pub mod tests {

    use crate::*;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Record)]
    pub struct Student {
        pub id: Option<RecordId>,
        pub name: String,
    }

    impl Student {
        pub fn new(id: RecordId, name: &str) -> Student {
            Student {
                id: Some(id),
                name: name.to_string(),
            }
        }
    }
}
