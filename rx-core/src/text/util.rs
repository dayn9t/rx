use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub type IoResult<T> = std::io::Result<T>;

/// 读取文本文件到字符串
pub fn read_text(file: impl AsRef<Path>) -> IoResult<String> {
    let mut txt = String::new();
    let mut f = File::open(file)?;
    f.read_to_string(&mut txt)?;
    Ok(txt)
}

/// 写入字符串到文本文件
pub fn write_text(file: impl AsRef<Path>, txt: String) -> IoResult<()> {
    let mut file = File::create(file)?;
    file.write_all(&txt.into_bytes())
}

/// 按照列表条目，替换字符串中的全部文本
pub fn replace_map(src: &str, map: &[(String, String)]) -> String {
    let mut txt = src.to_owned();
    for (from, to) in map {
        txt = txt.replace(from, to);
    }
    txt
}

/// 按照列表条目，替换文件中的全部文本，并写入目标文件
pub fn file_replace_map(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    map: &[(String, String)],
) -> IoResult<()> {
    let mut txt = read_text(src)?;
    txt = replace_map(&txt, map);
    write_text(dst, txt)
}

#[test]
fn test_all() {
    let file1 = "/tmp/rx-text-util-1.txt";
    let file2 = "/tmp/rx-text-util-2.txt";
    let src = "X+Y=Z\nX-Y=Z";
    let dst = "1+2=3\n1-2=3";
    write_text(file1, src.to_string()).unwrap();
    let txt1 = read_text(file1).unwrap();
    assert_eq!(src, txt1);

    let map = [
        ("X".to_string(), "1".to_string()),
        ("Y".to_string(), "2".to_string()),
        ("Z".to_string(), "3".to_string()),
    ];
    let txt2 = replace_map(&src, &map);
    assert_eq!(txt2, dst);

    file_replace_map(file1, file2, &map).unwrap();
    let txt1 = read_text(file2).unwrap();
    assert_eq!(txt1, dst);
}
