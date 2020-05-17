#[cfg(test)]
pub mod tests {

    use serde_derive::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Student {
        pub number: i32,
        pub name: String,
    }

    impl Student {
        pub fn new(number: i32, name: &str) -> Student {
            Student {
                number,
                name: name.to_string(),
            }
        }
    }
}
