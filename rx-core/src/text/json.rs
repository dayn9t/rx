use std::fs::File;
use std::io::{BufReader, BufWriter};

pub use serde_json::from_str;
pub use serde_json::to_string_pretty as to_pretty;

use crate::serde_export::*;

pub use super::basic::*;

/// 从JSON文件加载类型
pub fn load<T, P>(path: P) -> BoxResult<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let v = serde_json::from_reader(reader)?;
    Ok(v)
}

/// 对象保存到JSON文件
pub fn save<T, P>(value: &T, path: P) -> BoxResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}
