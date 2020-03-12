use encoding_rs::*;

/// GBK 转 UTF8
pub fn gbk_to_utf8_str(text: &[u8]) -> Option<String> {
    let (cow, _, had_err) = GBK.decode(text);
    if had_err {
        //        println!("Had err!");
    }
    Some(cow.to_string())
}

/// GBK 转 UTF8
pub fn gbk_to_utf8(text: &[u8]) -> Option<Vec<u8>> {
    let (cow, _, had_err) = GBK.decode(text);
    if had_err {
        //        println!("Had err!");
    }
    Some(cow.as_bytes().to_vec())
}
