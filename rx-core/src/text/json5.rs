use std::fs::File;
use std::io::BufWriter;
use std::io::{Read, Write};

pub use serde_json5::from_str;
pub use serde_json5::to_string as to_pretty;

use crate::serde_export::*;

pub use super::basic::*;

/// 从JSON文件加载类型
pub fn load<T, P>(path: P) -> BoxResult<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let mut f = File::open(path)?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s)?;
    let v = serde_json5::from_str(&s)?;
    Ok(v)
}

/// 对象保存到JSON文件
pub fn save<T, P>(value: &T, path: P) -> BoxResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let mut writer = BufWriter::new(File::create(path)?);
    let s = to_pretty(value)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

#[test]
fn io() {
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct A {
        i: u32,
        f: u32,
    }
    let a = A { i: 1, f: 2 };

    let s = r"{i:1,f:2}";
    let a1: A = from_str(s).unwrap();
    assert_eq!(a1, a);

    let s1 = to_pretty(&a).unwrap();

    assert_eq!(s1, "k");
}
