use std::ffi::OsStr;
pub use std::fs::File;
use std::fs::{DirEntry, create_dir_all};
pub use std::io::*;
pub use std::path::{Path, PathBuf};

use chrono::prelude::*;

/// 时间转化为文件
pub fn time_to_file(dt: &DateTime<Local>, ext: &str) -> String {
    let filename = dt.format("%Y-%m-%d/%H-%M-%S%.3f.").to_string() + ext;
    filename
}

/// 当前时间转化为文件
pub fn now_to_file(ext: &str) -> String {
    let dt = Local::now();
    time_to_file(&dt, ext)
}

/// 文件作为字符串访问
pub fn to_str<P>(p: &P) -> &str
where
    P: AsRef<Path> + ?Sized,
{
    p.as_ref().to_str().unwrap()
}

/// 文件转换为字符串
pub fn to_string(p: impl AsRef<Path>) -> String {
    p.as_ref().to_str().unwrap().to_owned()
}

/// 获取文件名
pub fn file_name(p: impl AsRef<Path>) -> String {
    p.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
}

/// 获取扩展名，不包括＂.＂，比如＂jpg"，而不是".jpg"
pub fn file_ext(p: impl AsRef<Path>) -> String {
    p.as_ref().extension().unwrap().to_str().unwrap().to_owned()
}

/// 获取主干文件名(去掉扩展名)
pub fn file_stem(p: impl AsRef<Path>) -> String {
    p.as_ref().file_stem().unwrap().to_str().unwrap().to_owned()
}

/// 在文件名后面追加后缀
pub fn file_name_append(p: impl AsRef<Path>, s: &str) -> PathBuf {
    let stem = file_stem(p.as_ref());
    let ext = file_ext(p.as_ref());
    let name = format!("{}{}.{}", stem, s, ext);
    let parent = p.as_ref().parent().unwrap();
    parent.join(&name)
}

/// 获取递归目录中指定扩展名文件
pub fn find_file_by_ext(dir: impl AsRef<Path>, ext: impl AsRef<str>) -> Result<Vec<PathBuf>> {
    let ext = Some(OsStr::new(ext.as_ref()));
    let mut vec = Vec::new();
    visit_dirs(dir.as_ref(), &mut |p: &Path| {
        if p.is_file() && p.extension() == ext {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取递归目录中指定名称文件
pub fn find_file_by_name(dir: impl AsRef<Path>, name: impl AsRef<str>) -> Result<Vec<PathBuf>> {
    let name = Some(OsStr::new(name.as_ref()));
    let mut vec = Vec::new();
    visit_dirs(dir.as_ref(), &mut |p: &Path| {
        if p.is_file() && p.file_name() == name {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 判断路径是有有扩展名
pub fn has_extension(_dir_entry: &DirEntry, _ext: impl AsRef<str>) -> bool {
    false
}

/// 排序方式
pub enum SortOrder {
    None,
    Asc,
    Desc,
}

/// 获取目录中文件
pub fn files_in(
    dir: impl AsRef<Path>,
    ext: impl AsRef<str>,
    sort_order: SortOrder,
) -> Result<Vec<PathBuf>> {
    let ext = Some(OsStr::new(ext.as_ref()));
    let mut vec = Vec::new();
    visit_dir(dir, &mut |p: &Path| {
        if p.is_file() && p.extension() == ext {
            vec.push(p.to_owned());
        }
    })?;
    match sort_order {
        SortOrder::Asc => vec.sort(),
        SortOrder::Desc => vec.sort_by(|a, b| b.cmp(a)),
        SortOrder::None => {}
    }
    Ok(vec)
}

/// 获取目录中文件名
pub fn file_names_in(
    dir: impl AsRef<Path>,
    ext: impl AsRef<str>,
    sort_order: SortOrder,
) -> Result<Vec<String>> {
    let v = files_in(dir, ext, sort_order)?;
    let v: Vec<_> = v.iter().map(|p| file_name(p)).collect();
    Ok(v)
}

/// 获取目录中文件名主干(去掉扩展名)
pub fn file_stems_in(
    dir: impl AsRef<Path>,
    ext: impl AsRef<str>,
    sort_order: SortOrder,
) -> Result<Vec<String>> {
    let v = files_in(dir, ext, sort_order)?;
    let v: Vec<_> = v.iter().map(|p| file_stem(p)).collect();
    Ok(v)
}

/// 文件复制到目录
pub fn copy_file_to_dir(src_file: impl AsRef<Path>, dst_dir: impl AsRef<Path>) -> Result<PathBuf> {
    let src_file = src_file.as_ref();
    let dst_dir = dst_dir.as_ref();
    let file_name = file_name(src_file);
    let dst_file = dst_dir.join(file_name);
    create_dir_all(&dst_dir).unwrap();
    std::fs::copy(src_file, &dst_file)?;
    Ok(dst_file)
}

/// 合并目录内所有文件到一个文件
pub fn combine_files_in(src_dir: &Path, dst_file: &Path, ext: &str) -> Result<()> {
    let files = files_in(&src_dir, &ext, SortOrder::Asc)?;
    combine_files(&files, dst_file)
}

/// 合并文件集合到一个文件
pub fn combine_files(src_files: &Vec<PathBuf>, dst_file: &Path) -> Result<()> {
    make_parent(&dst_file)?;
    let mut dst_file = File::create(dst_file)?;

    for file in src_files {
        println!("open: {:?}", file);
        let mut file = File::open(file)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        dst_file.write_all(&buf)?;
    }
    Ok(())
}

use crate::sys::fs::{make_parent, visit_dir, visit_dirs};
use tempfile::Builder;

/// 生成指定扩展名的临时文件
pub fn temp_file_with(ext: &str) -> PathBuf {
    let file = Builder::new()
        .suffix(&format!(".{}", ext))
        .tempfile()
        .unwrap();
    file.path().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A_JSON: &str = "/tmp/a.json";

    #[test]
    fn ext_works() {
        let ext = file_ext(&A_JSON);
        assert_eq!(ext, "json");
    }

    #[test]
    fn stem_works() {
        let name = file_name(&A_JSON);
        let stem = file_stem(&A_JSON);
        assert_eq!(name, "a.json");
        assert_eq!(stem, "a");
    }

    #[test]
    fn file_name_append_works() {
        let f1 = PathBuf::from(&A_JSON);
        let f2 = PathBuf::from("/tmp/a_1.json");
        let p = file_name_append(&f1, "_1");
        assert_eq!(f2, p);
    }

    #[test]
    fn make_parent_works() {
        let p = "/etc/passwd";
        assert_eq!(make_parent(&p).is_ok(), true);

        let p = "/etc/passwd/abc";
        assert_eq!(make_parent(&p).is_ok(), false);
    }
}
