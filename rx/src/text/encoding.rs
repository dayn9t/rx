use encoding_rs::*;

/// GBK 转 UTF8
pub fn gbk_to_utf8(opt_str: Option<&[u8]>) -> Option<String> {
    if let Some(s) = opt_str {
        let (cow, _, _) = GBK.decode(s);
        Some(cow.to_string())
    } else {
        None
    }
}
