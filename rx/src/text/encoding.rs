use encoding_rs::*;

/// 查找GBK编码标志
pub fn find_gbk(data: &[u8]) -> bool {
    let gbks = [
        "content=\"text/html; charset=gbk\"",
        "<meta charset=\"gbk\"",
    ];
    for s in &gbks {
        let s = s.as_bytes();
        let found = data.windows(s.len()).any(|w| w == s);
        if found {
            return true;
        }
    }
    false
}

/// GBK 转 UTF8
pub fn gbk_to_utf8_opt(opt_str: Option<&[u8]>) -> Option<String> {
    if let Some(s) = opt_str {
        let (cow, _, _) = GBK.decode(s);
        Some(cow.to_string())
    } else {
        None
    }
}

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
