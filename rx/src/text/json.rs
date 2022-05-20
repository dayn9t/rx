use std::fs::File;
pub use std::io::Result;
use std::io::{BufReader, BufWriter};
pub use std::path::Path;

pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_json::to_string_pretty as to_json;

pub use serde_json::from_str;

/// 从JSON文件加载类型
pub fn load_json<T, P>(path: P) -> Result<T>
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
pub fn save_json<T, P>(value: &T, path: P) -> Result<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}
