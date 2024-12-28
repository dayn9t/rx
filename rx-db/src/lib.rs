pub use interface::*;
pub use rx_core::prelude::*;
pub use rx_db_derive::Record;

//#![feature(associated_type_defaults)]
pub mod dirdb;

mod interface;
//pub mod redisdb;
mod test;

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {}
}
