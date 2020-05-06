#[macro_use]
mod macros;

#[macro_use]
extern crate serde_derive;

pub mod algo;
pub mod fs;
pub mod id;
pub mod text;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
