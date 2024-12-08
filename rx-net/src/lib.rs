#![feature(new_range_api)]
extern crate core;

pub mod cfg;
pub mod frp;
pub mod http;
pub mod ip;
pub mod mqtt;
pub mod prelude;
pub mod url;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
