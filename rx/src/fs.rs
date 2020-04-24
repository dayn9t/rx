use dirs;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
pub use std::io::{Error, ErrorKind, Result};
pub use std::path::{Path, PathBuf};

/// 获取文件名
pub fn file_name<P>(p: &P) -> &str
where
    P: AsRef<Path>,
{
    p.as_ref().file_name().unwrap().to_str().unwrap()
}

/// 获取主干文件名(去掉扩展名)
pub fn file_stem<P>(p: &P) -> &str
where
    P: AsRef<Path>,
{
    p.as_ref().file_stem().unwrap().to_str().unwrap()
}

/// 路径连接
pub fn join<P1, P2>(p1: &P1, p2: &P2) -> PathBuf
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
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
    join(&dirs::config_dir().unwrap(), &name.as_ref())
}

/// 确保目录存在
pub fn ensure_dir_exist<P>(path: &P) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = PathBuf::from(path.as_ref());
    if !p.exists() {
        let e = fs::create_dir_all(&p);
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
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/// 遍历目录
pub fn visit_dir<P>(dir: &P, cb: &mut dyn FnMut(&DirEntry)) -> Result<()>
where
    P: AsRef<Path>,
{
    for entry in fs::read_dir(dir)? {
        cb(&entry?);
    }
    Ok(())
}

/// 判断路径是有有扩展名
pub fn has_extension<S>(dir_entry: &DirEntry, ext: &S) -> bool
where
    S: AsRef<str>,
{
    false
}

/// 获取目录中文件
pub fn files_in<P, S>(dir: &P, ext: &S) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut files = Vec::new();
    let ext = Some(OsStr::new(ext.as_ref()));

    match fs::read_dir(dir) {
        Ok(v) => {
            for entry in v {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() && path.extension() == ext {
                    files.push(path);
                }
            }
            files
        }
        Err(_) => files,
    }
}

/// 获取目录中文件名
pub fn filenames_in<P, S>(dir: &P, ext: &S) -> Result<Vec<String>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut names = Vec::new();
    let ext = Some(OsStr::new(ext.as_ref()));

    visit_dir(dir, &mut |e: &DirEntry| {
        let path = e.path();
        if path.is_file() && path.extension() == ext {
            names.push(file_name(&path).to_string());
        }
    })?;
    Ok(names)
}

/// 获取目录中文件名主干(去掉扩展名)
pub fn filestems_in<P, S>(dir: &P, ext: &S) -> Result<Vec<String>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut names = Vec::new();
    let ext = Some(OsStr::new(ext.as_ref()));

    visit_dir(dir, &mut |e: &DirEntry| {
        let path = e.path();
        if path.is_file() && path.extension() == ext {
            names.push(file_stem(&path).to_string());
        }
    })?;
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_works() {
        let s1 = "/usr";
        let s2 = "bin";

        let p1 = PathBuf::from("/usr/bin");
        let p2 = join(&s1, &s2);
        let p3 = join(&"/usr", &"bin");
        let p3 = join(&s1, &"bin");

        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }

    #[test]
    fn stem_works() {
        let f1 = "/var/ias/a.json";
        let name = file_name(&f1);
        let stem = file_stem(&f1);
        assert_eq!(name, "a.json");
        assert_eq!(stem, "a");
    }
}
