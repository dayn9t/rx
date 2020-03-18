mod interface;
pub use self::interface::*;

#[macro_use]
extern crate serde_derive;

extern crate rx;

pub mod dirdb;
//pub mod leveldb

#[test]
fn it_works() {
    #[derive(Clone, Serialize, Deserialize)]
    struct Student {
        number: i32,
        name: String,
    }
    let s1 = Student {
        number: 1,
        name: "John".to_string(),
    };

    let rs1 = Record { id: 100, data: s1 };
}
