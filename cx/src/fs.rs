
pub use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};
use std::ffi::OsStr;
use std::io;


/// 确保目录存在
pub fn ensure_dir_exist<P: AsRef<Path>>(path: &P) -> io::Result<()> {
    let p = PathBuf::from(path.as_ref());
    if !p.exists() {
        let e = fs::create_dir_all(&p);
    }
    if p.is_dir() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::AlreadyExists, "path not a dir!"))
    }
}

/// 遍历目录访问文件
pub fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                try!(visit_dirs(&path, cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/// 判断路径是有有扩展名
pub fn has_extension<S: AsRef<str>>(dir_entry: &DirEntry, ext: &S) -> bool {
    false
}

/// 目录中文件
pub fn files_in<S: AsRef<str>>(dir: &Path, ext: &S) -> Vec<PathBuf> {
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
fn it_works() {
    let db = DirDb::open("db");
}
