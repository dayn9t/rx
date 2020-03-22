use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
pub use std::path::{Path, PathBuf};

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

/// 确保目录存在
pub fn ensure_dir_exist<P>(path: &P) -> io::Result<()>
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
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "path not a dir!",
        ))
    }
}

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
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

/// 判断路径是有有扩展名
pub fn has_extension<S>(dir_entry: &DirEntry, ext: &S) -> bool
where
    S: AsRef<str>,
{
    false
}

/// 目录中文件
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
