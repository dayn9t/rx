use std::ffi::OsStr;
pub use std::fs::File;
use std::fs::{self, DirEntry};
pub use std::io::*;
pub use std::path::{Path, PathBuf};

use dirs;

/// 文件作为字符串访问
pub fn to_str<P>(p: &P) -> &str
where
    P: AsRef<Path> + ?Sized,
{
    p.as_ref().to_str().unwrap()
}

/// 文件作为字符串访问
pub fn to_string<P>(p: &P) -> String
where
    P: AsRef<Path> + ?Sized,
{
    to_str(p).to_owned()
}

/// 获取文件名
pub fn file_name(p: &impl AsRef<Path>) -> &str {
    p.as_ref().file_name().unwrap().to_str().unwrap()
}

/// 获取文件名
pub fn file_name_owned<P>(p: &P) -> String
where
    P: AsRef<Path>,
{
    file_name(p).to_owned()
}

/// 获取扩展名，不包括＂.＂，比如＂jpg"，而不是".jpg"
pub fn file_ext<P>(p: &P) -> &str
where
    P: AsRef<Path>,
{
    p.as_ref().extension().unwrap().to_str().unwrap()
}

/// 获取扩展名，参见: file_ext
pub fn file_ext_owned<P>(p: &P) -> String
where
    P: AsRef<Path>,
{
    file_ext(p).to_owned()
}

/// 获取主干文件名(去掉扩展名)
pub fn file_stem<P>(p: &P) -> &str
where
    P: AsRef<Path>,
{
    p.as_ref().file_stem().unwrap().to_str().unwrap()
}

/// 获取主干文件名(去掉扩展名)
pub fn file_stem_owned<P>(p: &P) -> String
where
    P: AsRef<Path>,
{
    file_stem(p).to_owned()
}

/// 在文件名后面追加后缀
pub fn file_name_append<P>(p: &P, s: &str) -> PathBuf
where
    P: AsRef<Path>,
{
    let stem = file_stem(p);
    let ext = file_ext(p);
    let name = format!("{}{}.{}", stem, s, ext);
    let parent = p.as_ref().parent().unwrap();
    parent.join(&name)
}

/// 路径连接
pub fn join<P1, P2>(p1: &P1, p2: &P2) -> PathBuf
where
    P1: AsRef<Path> + ?Sized,
    P2: AsRef<Path> + ?Sized,
{
    let mut p = p1.as_ref().to_owned();
    p.push(p2.as_ref());
    p
}

/// 配置目录
pub fn config_dir_of<S>(name: S) -> PathBuf
where
    S: AsRef<str>,
{
    dirs::config_dir().unwrap().join(&name.as_ref())
}

/// 创建上级目录，幂等
pub fn make_parent<P>(path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::NotFound, "parent not found"))
    }
}

/// 确保目录存在，不存在则建立
pub fn ensure_dir_exist<P>(path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = path.as_ref();
    if !p.exists() {
        fs::create_dir_all(&p)?;
    }
    if p.is_dir() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::AlreadyExists, "path not a dir!"))
    }
}

/// 删除路径（文件/目录）
pub fn remove<P>(path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    } else {
        Ok(())
    }
}

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry.path());
            }
        }
    }
    Ok(())
}

/// 获取递归目录中指定扩展名文件
pub fn find_file_by_ext<P, S>(dir: &P, ext: &S) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
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
pub fn find_file_by_name<P, S>(dir: &P, name: &S) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let name = Some(OsStr::new(name.as_ref()));
    let mut vec = Vec::new();
    visit_dirs(dir.as_ref(), &mut |p: &Path| {
        if p.is_file() && p.file_name() == name {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 遍历目录
pub fn visit_dir<P>(dir: &P, cb: &mut dyn FnMut(&Path)) -> Result<()>
where
    P: AsRef<Path>,
{
    for entry in fs::read_dir(dir)? {
        cb(&entry?.path());
    }
    Ok(())
}

/// 判断路径是有有扩展名
pub fn has_extension<S>(_dir_entry: &DirEntry, _ext: &S) -> bool
where
    S: AsRef<str>,
{
    false
}

/// 获取目录中的目录
pub fn dirs_in<P>(dir: &P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut vec = Vec::new();
    visit_dir(dir, &mut |p: &Path| {
        if p.is_dir() {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取目录中目录名
pub fn dir_names_in<P>(dir: &P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let v = dirs_in(dir)?;
    let v: Vec<_> = v.iter().map(|p| file_name_owned(p)).collect();
    Ok(v)
}

/// 查找MTP设备目录
pub fn mtp_dirs() -> Result<Vec<PathBuf>> {
    dirs_in(&"/run/user/1000/gvfs")
}

/// 查找第一个目录(广度优先)
pub fn find_first_dir<P, S>(dir: &P, dir_name: &S) -> Result<PathBuf>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir.as_ref())? {
        let p = entry?.path();
        if p.is_dir() {
            if file_name(&p) == dir_name.as_ref() {
                return Ok(p);
            } else {
                dirs.push(p);
            }
        }
    }

    for dir in dirs {
        let r = find_first_dir(&dir, dir_name);
        if r.is_ok() {
            return r;
        }
    }
    Err(Error::from(ErrorKind::NotFound))
}

/// 获取目录中文件
pub fn files_in<P, S>(dir: &P, ext: &S) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let ext = Some(OsStr::new(ext.as_ref()));
    let mut vec = Vec::new();
    visit_dir(dir, &mut |p: &Path| {
        if p.is_file() && p.extension() == ext {
            vec.push(p.to_owned());
        }
    })?;
    Ok(vec)
}

/// 获取目录中文件名
pub fn file_names_in<P, S>(dir: &P, ext: &S) -> Result<Vec<String>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let v = files_in(dir, ext)?;
    let v: Vec<_> = v.iter().map(|p| file_name_owned(p)).collect();
    Ok(v)
}

/// 获取目录中文件名主干(去掉扩展名)
pub fn file_stems_in<P, S>(dir: &P, ext: &S) -> Result<Vec<String>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let v = files_in(dir, ext)?;
    let v: Vec<_> = v.iter().map(|p| file_stem_owned(p)).collect();
    Ok(v)
}

/// 合并目录内所有文件到一个文件
pub fn combine_files_in(src_dir: &Path, dst_file: &Path, ext: &str) -> Result<()> {
    let mut files = files_in(&src_dir, &ext)?;
    files.sort();
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

#[cfg(test)]
mod tests {
    use super::*;

    const A_JSON: &str = "/tmp/a.json";

    #[test]
    fn join_works() {
        let s1 = "/usr";
        let s2 = "bin";

        let p1 = PathBuf::from("/usr/bin");
        let p2 = join(&s1, &s2);
        let _p3 = join(&"/usr", &"bin");
        let p3 = join(&s1, &"bin");

        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }

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
