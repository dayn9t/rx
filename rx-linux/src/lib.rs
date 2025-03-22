#![feature(type_alias_impl_trait)]

pub mod command;
pub const RX_LINUX_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
