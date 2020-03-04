use encoding_rs::*;

/// GBK 转 UTF8
pub fn to_utf8(text: &[u8]) -> Option<String> {
    if let Ok(s) = std::str::from_utf8(text) {
        println!("encoding: UTF-8");
        return Some(s.to_string());
    }

    let (cow, _, had_err) = GBK.decode(text);
    if had_err {
        println!("Had err!");
    }
    //None

    Some(cow.to_string())
}
