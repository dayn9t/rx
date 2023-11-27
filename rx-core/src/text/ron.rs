use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_ron::de::from_str as from_serde_ron;
pub use serde_ron::ser::to_string;
use serde_ron::ser::{to_string_pretty, PrettyConfig};

pub use super::basic::*;

//use serde_ron::value::Value;//Serializer
//use std::io::Read;

/// 从RON文件加载类型
pub fn load<T, P>(path: P) -> BoxResult<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let v = serde_ron::de::from_reader(reader)?;
    Ok(v)
}

/// 对象保存到RON文件
pub fn save<T, P>(value: &T, path: P) -> BoxResult<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let writer = BufWriter::new(File::create(path)?);
    to_writer_pretty(writer, value)?;
    Ok(())
}

/// RON以美化格式表示为字符串
pub fn to_pretty<T>(value: &T) -> BoxResult<String>
where
    T: Serialize,
{
    let mut pretty = PrettyConfig::default();
    pretty.indentor = "\t".to_string();
    let s = to_string_pretty(&value, pretty)?;
    Ok(s)
}

/// RON以美化格式字符串写入
pub fn to_writer_pretty<W, T>(mut writer: W, value: &T) -> BoxResult<()>
where
    W: io::Write,
    T: Serialize,
{
    let s = to_pretty(&value)?;
    writer.write(&s.into_bytes())?;

    Ok(())
}
/*
/// RON格式化，结果居然是Json？
pub fn fmt(s: &str) -> BoxResult<String> {
    let value = Value::from_str(s)?;

    let mut pretty = PrettyConfig::default();
    pretty.indentor = "\t".to_string();
    let _s = to_string_pretty(&value, pretty)?;

    let mut ser = Serializer::new(None, true);
    value.serialize(&mut ser)?;
    let s = ser.into_output_string();
    //let s = to_pretty(&value)?;
    Ok(s)
}

/// RON格式化
pub fn fmt_file(f: &Path) -> BoxResult<String> {
    let mut f = File::open(f)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    fmt(&s)
}

#[test]
fn test_fmt() {
    let data = r#"
        ( // class name is optional
            materials: { // this is a map
                "metal": (
                    reflectivity: 1.0,
                ),
                "plastic": (
                    reflectivity: 0.5,
                ),
            },
            entities: [ // this is an array
                (
                    name: "hero",
                    material: "metal",
                ),
                (
                    name: "monster",
                    material: "plastic",
                ),
            ],
        )
        "#;
    let s = fmt(data).unwrap();
    assert_eq!(s, "");
}

#[test]
fn test_fmt1() {
    let data = "(name: 1, material: 1.1)";
    let s = fmt(data).unwrap();
    assert_eq!(s, "a");
}
*/
