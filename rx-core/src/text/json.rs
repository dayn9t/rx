use std::fs::File;
use std::io::{BufReader, BufWriter};

pub use serde_json::from_str;
pub use serde_json::to_string_pretty as to_pretty;

pub use super::basic::*;
use crate::serde_export::*;
use crate::sys::fs::make_parent;

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
    make_parent(&path)?;
    let writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::now;
    use chrono::Local;
    #[test]
    fn test_local_time_str() {
        let t1 = now();
        let s1 = to_pretty(&t1).unwrap();

        let t2 = Local::now();
        let s2 = to_pretty(&t2).unwrap();
        println!("time: {} {}", s1, s2);
    }
}
