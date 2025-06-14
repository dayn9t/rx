pub use interface::*;
pub use rx_core::prelude::*;
pub use rx_db_derive::Record;
pub use util::*;
//#![feature(associated_type_defaults)]
pub mod dirdb;

mod interface;
//pub mod redisdb;
mod test;
mod util;

pub const RX_DB_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {}
}
