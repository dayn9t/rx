#[macro_use]
mod macros;

pub mod algo;
pub mod collections;
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
