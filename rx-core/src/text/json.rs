use std::fs::File;
use std::io::{BufReader, BufWriter};

use serde_json::Serializer;
pub use serde_json::from_str;
use serde_json::ser::PrettyFormatter;
pub use serde_json::to_string;
pub use serde_json::to_string_pretty as to_pretty;

use crate::prelude::*;
use crate::sys::fs::make_parent;

/// 从JSON文件加载类型
pub fn load<T, P>(path: P) -> AnyResult<T>
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
pub fn save<T, P>(value: &T, path: P) -> AnyResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    make_parent(&path)?;
    let writer = BufWriter::new(File::create(path)?);
    let formatter = PrettyFormatter::with_indent(b"    "); // 4 spaces
    let mut ser = Serializer::with_formatter(writer, formatter);
    value.serialize(&mut ser)?;
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
