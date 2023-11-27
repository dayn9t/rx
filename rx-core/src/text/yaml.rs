use std::fs::File;
use std::io::{BufReader, BufWriter};

pub use serde_yaml::from_str;
pub use serde_yaml::to_string;

use crate::serde_export::*;

pub use super::basic::*;

/// 从YAML文件加载类型
pub fn load<T, P>(path: P) -> BoxResult<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let v = serde_yaml::from_reader(reader)?;
    Ok(v)
}

/// 对象保存到yaml文件
pub fn save<T, P>(value: &T, path: P) -> BoxResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let writer = BufWriter::new(File::create(path)?);
    serde_yaml::to_writer(writer, value)?;
    Ok(())
}
