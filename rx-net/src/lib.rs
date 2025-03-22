#![feature(new_range_api)]
extern crate core;

pub mod cfg;
pub mod frp;
pub mod http;
pub mod ip;
pub mod mqtt;
pub mod prelude;
pub mod url;

pub const RX_NET_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
