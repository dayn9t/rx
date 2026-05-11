pub mod api;
pub mod task;

pub const RX_REST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
