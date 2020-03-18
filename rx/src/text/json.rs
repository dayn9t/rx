pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
pub use serde_json::to_string_pretty as to_json;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

/// 从JSON文件加载类型
pub fn load_json<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> io::Result<T> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let v: T = serde_json::from_str(&buf)?;
    Ok(v)
}
/*
fn load_json<P: AsRef<Path>, T>(path: P) -> Result<T, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}*/
