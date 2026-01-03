#![feature(step_trait)]
#![feature(new_range_api)]
extern crate alloc;
extern crate core;

#[macro_use]
mod macros;

pub mod algo;
pub mod app;
pub mod c;
pub mod collections;
pub mod complete_by;
pub mod encryptor;
pub mod id;
pub mod int_enum;
pub mod log;
pub mod m;
pub mod prelude;
pub mod sys;
/// 用于测试程序的模块
mod test;
pub mod text;
pub mod time;

pub const RX_CORE_DIR: &str = env!("CARGO_MANIFEST_DIR");
