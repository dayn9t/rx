
mod interface;
pub use self::interface::*;

pub mod dirdb;
pub mod leveldb;


#[test]
fn it_works() {

    #[derive(Clone, RustcDecodable, RustcEncodable)]
    struct Student {
        number: i32,
        name: String,
    }
    let s1 = Student {
        number: 1,
        name: "John",
    };

    let rs1 = Record {
        id: 100,
        data: s1,
    };
}
